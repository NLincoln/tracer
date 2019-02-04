use clap::{App, Arg};
use libtrace::scene::{Image, Rendered, Scene};
use png::HasParameters;
use std::{error::Error, fs::File, io::BufWriter, thread};

const SAMPLES_PER_WORKER: usize = 5;

fn main() -> Result<(), Box<dyn Error>> {
  let matches = App::new("Tracer Coordinator")
    .about("Raytrace an image in AWS lambda")
    .arg(
      Arg::with_name("worker-url")
        .help("URL to send jobs to")
        .required(true)
        .long("url")
        .takes_value(true)
        .value_name("URL"),
    )
    .arg(
      Arg::with_name("scene")
        .help("File to render")
        .default_value("scene.yml")
        .takes_value(true)
        .value_name("SCENE"),
    )
    .get_matches();

  let scene: Scene = serde_yaml::from_reader(File::open(matches.value_of("scene").unwrap())?)?;

  let num_workers = (scene.image.samples as f32 / SAMPLES_PER_WORKER as f32).ceil() as usize;

  let (sender, recver) = crossbeam_channel::unbounded();
  let mut threads = Vec::with_capacity(num_workers);
  let worker_url = matches.value_of("worker-url").unwrap().to_string();

  for _ in 0..num_workers {
    let sender = sender.clone();
    let worker_url = worker_url.clone();
    let scene = scene.clone();
    threads.push(thread::spawn(move || {
      let client = reqwest::Client::new();
      let worker_scene = Scene {
        image: Image {
          samples: SAMPLES_PER_WORKER as u32,
          ..scene.image
        },
        ..scene
      };
      let result = client
        .post(&worker_url)
        .json(&worker_scene)
        .send()
        .map_err(|err| err.to_string())
        .and_then(|mut response| {
          let rendered: Rendered = response.json().map_err(|err| err.to_string())?;
          Ok(rendered)
        });
      let _ = sender.send(result);
    }));
  }

  let mut results = Vec::with_capacity(num_workers);

  while results.len() < num_workers {
    results.push(recver.recv()??);
  }

  for thread in threads.into_iter() {
    thread.join().unwrap();
  }

  let mut result_image = Vec::with_capacity((scene.image.width * scene.image.height) as usize);

  for i in 0..(scene.image.height * scene.image.width) {
    let mut total_value = (0u32, 0u32, 0u32);
    for result in results.iter() {
      let (r, g, b) = result.pixels[i as usize];
      total_value.0 += r as u32;
      total_value.1 += g as u32;
      total_value.2 += b as u32;
    }
    result_image.push((
      (total_value.0 / results.len() as u32) as u8,
      (total_value.1 / results.len() as u32) as u8,
      (total_value.2 / results.len() as u32) as u8,
    ));
  }

  let output = File::create("render-result.png")?;
  let writer = BufWriter::new(output);
  let mut encoder = png::Encoder::new(writer, scene.image.width, scene.image.height);
  encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
  let mut writer = encoder.write_header()?;
  let mut image_data = Vec::with_capacity(result_image.len() * 4);

  for pixel in result_image.iter() {
    image_data.push(pixel.0);
    image_data.push(pixel.1);
    image_data.push(pixel.2);
    image_data.push(255);
  }

  writer.write_image_data(&image_data)?;

  Ok(())
}
