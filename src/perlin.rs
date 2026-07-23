use crate::rtweekend::*;

#[derive(Copy, Clone)]
pub struct Perlin {
    rand_vec: [Vec3; Self::POINT_COUNT],
    perm_x: [u64; Self::POINT_COUNT],
    perm_y: [u64; Self::POINT_COUNT],
    perm_z: [u64; Self::POINT_COUNT],
}

impl Perlin {
    pub const POINT_COUNT: usize = 256;

    pub fn new() -> Perlin {
        let mut p = Self::blank();
        for i in 0..Self::POINT_COUNT {
            p.rand_vec[i] = random_unit_vec();
        }

        Self::perlin_generate_perm(&mut p.perm_x);
        Self::perlin_generate_perm(&mut p.perm_y);
        Self::perlin_generate_perm(&mut p.perm_z);
        p
    }

    fn blank() -> Perlin {
        Perlin {
            rand_vec: [Vec3::new(0.0, 0.0, 0.0); Self::POINT_COUNT],
            perm_x: [0; Self::POINT_COUNT],
            perm_y: [0; Self::POINT_COUNT],
            perm_z: [0; Self::POINT_COUNT],
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i64;
        let j = p.y().floor() as i64;
        let k = p.z().floor() as i64;
        let mut c = [[[Vec3::empty(); 2]; 2]; 2];

        for di in 0..2_usize {
            for dj in 0..2_usize {
                for dk in 0..2_usize {
                    c[di][dj][dk] = self.rand_vec[(self.perm_x[((i + di as i64) & 255) as usize]
                        ^ self.perm_y[((j + dj as i64) & 255) as usize]
                        ^ self.perm_z[((k + dk as i64) & 255) as usize])
                        as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm(p: &mut [u64]) {
        for i in 0..Self::POINT_COUNT {
            p[i] = i as u64;
        }
        Self::permute(p, Self::POINT_COUNT);
    }

    fn permute(p: &mut [u64], n: usize) {
        for i in (1..n).rev() {
            let target = random_int(0, i);
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = Self::hermitian(u);
        let vv = Self::hermitian(v);
        let ww = Self::hermitian(w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;
                    let weight = Vec3::new(u - i_f, v - j_f, w - k_f);
                    accum += (i_f * uu + (1.0 - i_f) * (1.0 - uu))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - vv))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - ww))
                        * dot(&c[i][j][k], &weight);
                }
            }
        }
        accum
    }

    fn hermitian(n: f64) -> f64 {
        n * n * (3.0 - 2.0 * n)
    }

    pub fn turbulance(&self, p: &Point3, depth: i64) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p = 2.0 * temp_p;
        }

        accum.abs()
    }
}
