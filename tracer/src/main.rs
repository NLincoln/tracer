use std::f32;

use libtrace::{
    material::{Dialectric, Lambertian, Material, Metal},
    texture, BvhNode, Hitable, MovingSphere, StaticSphere, Vec3,
};

use indicatif::{ProgressBar, ProgressStyle};
use libtrace::texture::{CheckerBoard, Image, NoiseTexture, Texture};
use rand::prelude::*;
use rayon::prelude::*;
use std::error::Error;
use std::fs;

#[allow(unused)]
fn two_spheres() -> Hitable {
    use texture::Color;
    let checker: Texture =
        CheckerBoard::new(10., Color::new((0.2, 0.3, 0.1)), Color::new(0.9)).into();
    let world: Vec<Hitable> = vec![
        StaticSphere::new(10., (0., -10., 0.), Lambertian::new(checker.clone())).into(),
        StaticSphere::new(2.5, (0., 2.5, 0.), Lambertian::new(checker.clone())).into(),
    ];
    Hitable::List(world.into())
}

fn earth_sphere() -> Hitable {
    let text: Texture = Image::new(image::open("earth.jpg").unwrap()).into();
    StaticSphere::new(10., (0., 0., 0.), Lambertian::new(text)).into()
}

fn two_perlin_spheres() -> Hitable {
    let text: Texture = NoiseTexture::new(4.).into();
    //    let text: Texture = Color::new(0.).into();
    let world: Vec<Hitable> = vec![
        StaticSphere::new(1000., (0., -1000., 0.), Lambertian::new(text.clone())).into(),
        StaticSphere::new(2., (0., 2., 0.), Lambertian::new(text)).into(),
    ];
    Hitable::List(world.into())
}

fn random_scene() -> Hitable {
    use texture::Color;
    let mut world: Vec<Hitable> = Vec::new();

    fn metal(color: (f32, f32, f32), fuzz: f32) -> Material {
        Metal::new(Color::new(color), fuzz).into()
    }

    fn lambert(color: (f32, f32, f32)) -> Material {
        Lambertian::new(Color::new(color)).into()
    }

    world.push(
        StaticSphere::new(
            1000.,
            (0., -1000., 0.),
            Lambertian::new(CheckerBoard::new(
                10.,
                Color::new((0.8, 0.8, 0.8)),
                Color::new((0.2, 0.3, 0.1)),
            )),
        )
        .into(),
    );

    let mut rng = rand::thread_rng();

    for a in -3..=3 {
        for b in -3..=3 {
            let a = a as f32;
            let b = b as f32;
            let material_type: f32 = rng.gen();
            let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }
            let material: Material = if material_type < 0.8 {
                lambert((
                    rng.gen::<f32>() * rng.gen::<f32>(),
                    rng.gen::<f32>() * rng.gen::<f32>(),
                    rng.gen::<f32>() * rng.gen::<f32>(),
                ))
            } else if material_type < 0.95 {
                metal(
                    (
                        0.5 * (1. + rng.gen::<f32>()),
                        0.5 * (1. + rng.gen::<f32>()),
                        0.5 * (1. + rng.gen::<f32>()),
                    ),
                    0.5 * rng.gen::<f32>(),
                )
            } else {
                Dialectric::new(1.5).into()
            };

            let sphere: Hitable = if material_type < 0.8 {
                MovingSphere::new(
                    0.2,
                    material,
                    (0.0, center),
                    (1.0, center + Vec3::new(0., 0.5 * rng.gen::<f32>(), 0.)),
                )
                .into()
            } else {
                StaticSphere::new(0.2, center, material).into()
            };

            world.push(sphere);
        }
    }

    world.push(StaticSphere::new(1.0, (0., 1., 0.), Dialectric::new(1.5)).into());

    world.push(StaticSphere::new(1.0, (-4., 1., 0.), lambert((0.4, 0.2, 0.1))).into());

    world.push(StaticSphere::new(1.0, (4., 1., 0.), metal((0.7, 0.6, 0.5), 0.0)).into());

    Hitable::BvhNode(BvhNode::new(world, (0.0, 1.0)))
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
                .into_iter()
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
    scene.objects = earth_sphere();
    //    serde_yaml::to_writer(fs::File::create("scene.yml").unwrap(), &scene).unwrap();

    let progress_bar = ProgressBar::new(num_pixels as u64);

    progress_bar.set_style(ProgressStyle::default_bar().template(
        "[{elapsed_precise} elapsed] {wide_bar:.green/white} {percent}% [{eta} remaining]",
    ));

    let renderer = WorkstationRenderer {
        progress_bar: &progress_bar,
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
