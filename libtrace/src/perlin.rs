use crate::Vec3;
use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use serde_derive::{Deserialize, Serialize};

fn perlin_generate() -> Vec<Vec3> {
    let mut rng = thread_rng();

    let mut buf = Vec::with_capacity(256);
    for _ in 0..256 {
        let vec = Vec3::from(|| rng.gen_range(-1., 1.)).into_normalized();
        buf.push(vec);
    }
    buf
}

fn perlin_generate_float() -> Vec<f32> {
    let mut rng = thread_rng();

    let mut buf: Vec<f32> = Vec::with_capacity(256);
    for _ in 0..256 {
        buf.push(rng.gen::<f32>());
    }
    buf
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p: Vec<_> = (0..256).collect();
    thread_rng().shuffle(&mut p);
    p
}

lazy_static! {
    static ref PERM_X: Vec<usize> = perlin_generate_perm();
    static ref PERM_Y: Vec<usize> = perlin_generate_perm();
    static ref PERM_Z: Vec<usize> = perlin_generate_perm();
    static ref RAN_VEC: Vec<Vec3> = perlin_generate();
    static ref RAN_FLOAT: Vec<f32> = perlin_generate_float();
}

fn turbulence(mut p: Vec3, depth: usize) -> f32 {
    let mut accum = 0.0;
    let mut weight = 1.0;
    for _ in 0..depth {
        accum += weight * noise(p);
        weight *= 0.5;
        p *= 2.;
    }
    accum.abs()
}

fn noise(p: Vec3) -> f32 {
    fn add_and_usize(a: i32, b: i32) -> usize {
        ((a + b) & 255) as usize
    }

    let (i, j, k) = p.apply(|v| v.floor()).to_tuple_and(|val| val as i32);

    let mut buf = [Vec3::from(0.0); 8];
    for di in 0..=1i32 {
        for dj in 0..=1i32 {
            for dk in 0..=1i32 {
                buf[(di * 4 + dj * 2 + dk) as usize] = RAN_VEC[PERM_X[add_and_usize(i, di)]
                    ^ PERM_Y[add_and_usize(j, dj)]
                    ^ PERM_Z[add_and_usize(k, dk)]]
            }
        }
    }
    perlin_interp(buf, p)
}

fn perlin_interp(c: [Vec3; 8], p: Vec3) -> f32 {
    let mut accum = 0.;
    let rounded = p.apply(|v| v - v.floor());
    let (u, v, w) = rounded.to_tuple();
    let (uu, vv, ww) = rounded.apply(|val| val * val * (3. - 2. * val)).to_tuple();

    for i in 0..=1 {
        let i_f = i as f32;
        for j in 0..=1 {
            let j_f = j as f32;
            for k in 0..=1 {
                let k_f = k as f32;

                let weight = Vec3::new(u - i_f, v - j_f, w - k_f);

                let c_vec = c[i * 4 + j * 2 + k];

                let term = (i_f * uu + (1. - i_f) * (1. - uu))
                    * (j_f * vv + (1. - j_f) * (1. - vv))
                    * (k_f * ww + (1. - k_f) * (1. - ww))
                    * c_vec.dot(weight);

                accum += term;
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

    pub(crate) fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
        (0.5 * (1. + (self.scale * p.z() + 10. * turbulence(p, 10)).sin())).into()
    }
}
