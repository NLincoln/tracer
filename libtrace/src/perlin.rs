use crate::Vec3;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};

fn perlin_generate() -> Vec<Vec3> {
    let mut rng = thread_rng();

    let mut buf = Vec::with_capacity(256);
    for i in 0..256 {
        let x = rng.gen_range(-1., 1.);
        let y = rng.gen_range(-1., 1.);
        let z = rng.gen_range(-1., 1.);

        buf.push(Vec3::new(x, y, z).into_normalized());
    }
    buf
}

fn perlin_generate_float() -> Vec<f32> {
    let mut rng = thread_rng();

    let mut buf: Vec<f32> = Vec::with_capacity(256);
    for i in 0..256 {
        buf.push(rng.gen());
    }
    buf
}

fn permute(p: &mut [usize]) {
    let mut i = p.len() - 1;
    for i in 0..p.len() {
        let target = thread_rng().gen_range(i, p.len());
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p: Vec<_> = (0..256).into_iter().collect();
    permute(&mut p);
    p
}

lazy_static! {
    static ref PERM_X: Vec<usize> = perlin_generate_perm();
    static ref PERM_Y: Vec<usize> = perlin_generate_perm();
    static ref PERM_Z: Vec<usize> = perlin_generate_perm();
    static ref RAN_VEC: Vec<Vec3> = perlin_generate();
    static ref RAN_FLOAT: Vec<f32> = perlin_generate_float();
}

fn noise(p: Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = ((4. * p.x()) as isize & 0xff) as usize;
    let j = ((4. * p.y()) as isize & 0xff) as usize;
    let k = ((4. * p.z()) as isize & 0xff) as usize;

    RAN_FLOAT[PERM_X[i] ^ PERM_Y[j] ^ PERM_Z[k]]
}

fn trilinear_interp(c: [f32; 8], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.;

    for i in 0..2 {
        let i_f = i as f32;
        for j in 0..2 {
            let j_f = j as f32;
            for k in 0..2 {
                let k_f = k as f32;

                accum += (i_f * u + (1. - i_f) * (1. - u))
                    * (j_f * v + (1. - j_f) * (1. - v))
                    * (k_f * w + (1. - k_f) * (1. - w))
                    * c[i * 4 + j * 2 + k];
            }
        }
    }
    accum
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NoiseTexture {
    scale: f32,
}

impl Default for NoiseTexture {
    fn default() -> NoiseTexture {
        NoiseTexture { scale: 1.0 }
    }
}

impl NoiseTexture {
    pub fn new(scale: f32) -> NoiseTexture {
        NoiseTexture { scale }
    }

    pub(crate) fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
        noise(p * self.scale).into()
    }
}
