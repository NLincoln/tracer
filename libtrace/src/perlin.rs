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

fn permute(p: &mut [usize]) {
    let mut i = p.len() - 1;
    while i > 0 {
        let target = (thread_rng().gen::<f32>() * (i as f32 + 1.)) as usize;
        p.swap(i, target);
        i -= 1;
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
}

fn noise(p: Vec3) -> f32 {
    let u = p.x() - p.x().floor();
    let v = p.y() - p.y().floor();
    let w = p.z() - p.z().floor();

    let i = p.x().floor() as isize;
    let j = p.y().floor() as isize;
    let k = p.z().floor() as isize;

    let mut buf = [Vec3::from(0.); 8];
    for di in 0..2isize {
        for dj in 0..2isize {
            for dk in 0..2isize {
                let x_term  = ((i + di) & 255) as usize;
                let y_term  = ((j + dj) & 255) as usize;
                let z_term  = ((k + dk) & 255) as usize;

                buf[(di * 4 + dj * 2 + dk) as usize] = RAN_VEC
                    [PERM_X[x_term] ^ PERM_Y[y_term] ^ PERM_Z[z_term]]
            }
        }
    }

    trilinear_interp(buf, u, v, w)
}

#[inline]
fn trilinear_interp(c: [Vec3; 8], u: f32, v: f32, w: f32) -> f32 {
    let mut accum = 0.;
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);

    for i in 0..2 {
        let i_f = i as f32;
        for j in 0..2 {
            let j_f = j as f32;
            for k in 0..2 {
                let k_f = k as f32;

                let weight: Vec3 = (u - i_f, v - j_f, w - k_f).into();
                accum += (i_f * uu + (1. - i_f) * (1. - uu))
                    * (j_f * vv + (1. - j_f) * (1. - vv))
                    * (k_f * ww + (1. - k_f) * (1. - ww))
                    * c[i * 4 + j * 2 + k].dot(weight);
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
