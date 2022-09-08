use std::ops::BitAnd;

use crate::types::Point3;
use crate::utils;
use crate::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Default for Perlin {
    fn default() -> Self {
        let mut ranvec = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            ranvec.push(utils::rand_vec3_range(-1.0, 1.0));
        }
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Self {
            ranvec,
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

    pub fn noise(&self, p: Point3) -> crate::Float {
        let max = (POINT_COUNT - 1) as i64;

        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i64;
        let j = p.y.floor() as i64;
        let k = p.z.floor() as i64;

        let mut color_v: [[[Vec3; 2]; 2]; 2] = Default::default();

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    color_v[di][dj][dk] = self.ranvec[self.perm_x
                        [((i + di as i64).bitand(max)) as usize]
                        ^ self.perm_y[((j + dj as i64).bitand(max)) as usize]
                        ^ self.perm_z[((k + dk as i64).bitand(max)) as usize]]
                }
            }
        }

        return Self::trilinear_interp(color_v, u, v, w);
    }

    pub fn turb(&self, p: Point3, depth: u32) -> crate::Float {
        let mut accum = 0.0;
        let mut p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(p);
            weight *= 0.5;
            p *= 2.0;
        }

        return accum.abs();
    }

    fn trilinear_interp(
        color_v: [[[Vec3; 2]; 2]; 2],
        u: crate::Float,
        v: crate::Float,
        w: crate::Float,
    ) -> crate::Float {
        let mut accumulate = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as crate::Float;
                    let fj = j as crate::Float;
                    let fk = k as crate::Float;

                    let weight = (fi * u + (1.0 - fi) * (1.0 - u))
                        * (fj * v + (1.0 - fj) * (1.0 - v))
                        * (fk * w + (1.0 - fk) * (1.0 - w));

                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);

                    accumulate += color_v[i][j][k].dot(weight_v) * weight;
                }
            }
        }
        accumulate
    }
}
