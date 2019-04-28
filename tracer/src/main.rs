use std::f32;

use libtrace::{
    material::{Dialectric, Lambertian, Material, Metal},
    Hitable, StaticSphere, MovingSphere, Vec3,
};

use indicatif::{ProgressBar, ProgressStyle};
use rand::prelude::*;
use rayon::prelude::*;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
fn random_scene() -> Hitable {
    let mut world: Vec<Hitable> = Vec::new();
    world.push(
        StaticSphere::new(
            1000.,
            (0., -1000., 0.).into(),
            Lambertian::new((0.5, 0.5, 0.5).into()).into(),
        )
        .into(),
    );

    let mut rng = rand::thread_rng();

    for a in -11..=10 {
        for b in -11..=10 {
            let a = a as f32;
            let b = b as f32;
            let material_type: f32 = rng.gen();
            let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }
            let material: Material = if material_type < 0.8 {
                Lambertian::new(
                    (
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                        rng.gen::<f32>() * rng.gen::<f32>(),
                    )
                        .into(),
                )
                .into()
            } else if material_type < 0.95 {
                Metal::new(
                    (
                        0.5 * (1. + rng.gen::<f32>()),
                        0.5 * (1. + rng.gen::<f32>()),
                        0.5 * (1. + rng.gen::<f32>()),
                    )
                        .into(),
                    0.5 * rng.gen::<f32>(),
                )
                .into()
            } else {
                Dialectric::new(1.5).into()
            };

            let sphere: Hitable = if material_type < 0.8 {
                MovingSphere::new(0.2, material, (0.0, center), (1.0, center + Vec3::new(0., 0.5*rng.gen::<f32>(), 0.))).into()
            } else {
                StaticSphere::new(0.2, center, material).into()
            };

            world.push(sphere);
        }
    }

    world.push(StaticSphere::new(1.0, (0., 1., 0.).into(), Dialectric::new(1.5).into()).into());

    world.push(
        StaticSphere::new(
            1.0,
            (-4., 1., 0.).into(),
            Lambertian::new((0.4, 0.2, 0.1).into()).into(),
        )
        .into(),
    );

    world.push(
        StaticSphere::new(
            1.0,
            (4., 1., 0.).into(),
            Metal::new((0.7, 0.6, 0.5).into(), 0.0).into(),
        )
        .into(),
    );

    Hitable::List(world)
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap::App::new("Tracer")
        .version("0.1.0")
        .about("Ray traces an image")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .value_name("FILE")
                .takes_value(true)
                .default_value("image.png"),
        )
        .arg(
            clap::Arg::with_name("input")
                .short("i")
                .value_name("FILE")
                .takes_value(true)
                .default_value("scene.yml"),
        )
        .get_matches();

    use libtrace::{renderer::Renderer, scene::Scene};

    struct WorkstationRenderer<'a> {
        scene: &'a Scene,
        progress_bar: &'a ProgressBar,
    }

    impl<'a> Renderer for WorkstationRenderer<'a> {
        #[inline]
        fn scene(&self) -> &Scene {
            self.scene
        }
        fn render(&self) -> Vec<(u8, u8, u8)> {
            let scene = self.scene();
            let camera = self.camera(&scene);

            self.get_pixels_to_render(&scene)
                .into_par_iter()
                .map(|(i, j)| self.render_pixel(&camera, (i, j), &scene))
                .collect()
        }
        #[inline]
        fn on_pixel_rendered(&self, _location: (u32, u32), _color: (u8, u8, u8)) {
            self.progress_bar.inc(1);
        }
    }

    let mut scene: Scene =
        serde_yaml::from_reader(fs::File::open(matches.value_of("input").unwrap())?)?;
    let num_pixels = scene.image.num_pixels();
    scene.objects = random_scene();
    serde_yaml::to_writer(fs::File::create("scene.yml").unwrap(), &scene).unwrap();

    let bar = ProgressBar::new(num_pixels as u64);

    bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise} elapsed] {wide_bar:.green/white} {percent}% [{eta} remaining]",
    ));

    let renderer = WorkstationRenderer {
        progress_bar: &bar,
        scene: &scene,
    };

    let pixels = renderer.render();

    let mut output = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(matches.value_of("output").unwrap())?;

    renderer.write_image(&mut output, &pixels)?;
    Ok(())
}
