use std::ops::BitAnd;

use crate::utils;
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranfloat = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranfloat.push(utils::rand_f64());
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    fn perlin_generate_perm() -> Vec<usize> {
        let mut p = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            p.push(i);
        }
        Self::permute(&mut p);

        p
    }

    fn permute(p: &mut Vec<usize>) {
        for i in (1..(POINT_COUNT - 1)).rev() {
            let target = utils::gen_range(0..i);
            (p[i], p[target]) = (p[target], p[i]);
        }
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let max = (POINT_COUNT - 1) as i32;

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut color: [[[f64; 2]; 2]; 2] = Default::default();

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    color[di][dj][dk] = self.ranfloat[self.perm_x
                        [((i + di as i32).bitand(max)) as usize]
                        ^ self.perm_y[((j + dj as i32).bitand(max)) as usize]
                        ^ self.perm_z[((k + dk as i32).bitand(max)) as usize]]
                }
            }
        }

        return Self::trilinear_interp(color, u, v, w);
    }

    fn trilinear_interp(color: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accumulate = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;
                    let weight = (fi * u + (1.0 - fi) * (1.0 - u))
                        * (fj * v + (1.0 - fj) * (1.0 - v))
                        * (fk * w + (1.0 - fk) * (1.0 - w));
                    accumulate += color[i][j][k] * weight;
                }
            }
        }
        accumulate
    }
}
