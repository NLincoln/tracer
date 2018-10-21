pub mod ppm;
pub mod vec3;

fn main() {
  let nx = 200;
  let ny = 100;
  print!("P3\n{} {}\n255\n", nx, ny);

  for j in 0..ny {
    let j = ny - j;
    for i in 0..nx {
      let color = ppm::Color {
        red: i as f32 / nx as f32,
        green: j as f32 / ny as f32,
        blue: 0.2,
      };
      println!("{}", color);
    }
  }
}
