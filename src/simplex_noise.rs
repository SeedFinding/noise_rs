#![allow(dead_code)]

use java_random::Random;
use std::collections::HashMap;
use intmap::IntMap;
use crate::noise::Noise;
use crate::math::dot;

pub const F2: f64 = 0.3660254037844386;
pub const G2: f64 = 0.21132486540518713;
pub const F3: f64 = 0.3333333333333333;
pub const G3: f64 = 0.16666666666666666;
pub const GRADIENT: [[i32; 3]; 16] =
    [[1, 1, 0],
        [-1, 1, 0],
        [1, -1, 0],
        [-1, -1, 0],
        [1, 0, 1],
        [-1, 0, 1],
        [1, 0, -1],
        [-1, 0, -1],
        [0, 1, 1],
        [0, -1, 1],
        [0, 1, -1],
        [0, -1, -1],
        [1, 1, 0],
        [0, -1, 1],
        [-1, 1, 0],
        [0, -1, -1]];

#[derive(Clone,Debug)]
pub struct SimplexNoise {
    pub noise: Noise,
    pub cache2d: IntMap<f64>,
    pub cache3d: HashMap<u128, f64>,
}


#[cfg(test)]
mod simplex_test {
    use super::*;

    #[test]
    fn test_coordinates(){
        let random:Random=Random::with_seed(12);
        let simplex_noise:SimplexNoise=SimplexNoise::init(random);
        assert_eq!(simplex_noise.get_coordinates(),(186.85255836421052f64, 70.41770637313917f64, 123.13254179103222f64));
    }
    #[test]
    fn test_2d(){
        let random:Random=Random::with_seed(12);
        let mut simplex_noise:SimplexNoise=SimplexNoise::init(random);
        assert_eq!(simplex_noise.get_value_2d(0.5,100.0),0.8331228771221665);
    }
    #[test]
    fn test_3d(){
        let random:Random=Random::with_seed(12);
        let mut simplex_noise:SimplexNoise=SimplexNoise::init(random);
        assert_eq!(simplex_noise.get_value_3d(0.5,0.6,100.0),-0.047980544000000055);
    }
}

impl SimplexNoise {
    #[cold]
    pub fn new(noise: Noise) -> Self {
        SimplexNoise {
            noise,
            cache2d: IntMap::with_capacity(1024),
            cache3d: HashMap::new(),
        }
    }
    pub fn init(mut random: Random) -> SimplexNoise {
        SimplexNoise::new(Noise::new(&mut random))
    }

    fn lookup(&self, n: u8) -> u8 {
        self.noise.lookup(n as i32)
    }

    fn get_corner_noise3d(n: u8, x: f64, y: f64, z: f64, max: f64) -> f64 {
        let res: f64;
        let mut contribution: f64 = max - x * x - y * y - z * z;
        if contribution < 0.0 {
            res = 0.0;
        } else {
            contribution *= contribution;
            res = contribution * contribution * dot(GRADIENT[n as usize], x, y, z);
        }
        res
    }

    pub fn get_value_2d(&mut self, x: f64, z: f64) -> f64 {
        let key: u64 = (((x as u32) as u64) << 32 | ((z as u32) as u64)) as u64;
        let value: f64 = *self.cache2d.get(key).unwrap_or(&std::f64::MAX);
        if value != std::f64::MAX {
            return value;
        }
        let value: f64 = self._get_value_2d(x, z);
        self.cache2d.insert(key, value);
        return value;
    }

    fn _get_value_2d(&self, x: f64, z: f64) -> f64 {
        let hairy_factor: f64 = (x + z) * F2;
        let temperature_x: i32 = (x + hairy_factor).floor() as i32;
        let temperature_z: i32 = (z + hairy_factor).floor() as i32;
        let mixed_temperature_x_z: f64 = (temperature_x + temperature_z) as f64 * G2;
        let temp_diff_x_to_z: f64 = temperature_x as f64 - mixed_temperature_x_z;
        let temp_diff_z_to_x: f64 = temperature_z as f64 - mixed_temperature_x_z;
        let x0: f64 = x - temp_diff_x_to_z;
        let y0: f64 = z - temp_diff_z_to_x;
        let offset_second_corner_x: u8;
        let offset_second_corner_z: u8;

        if x0 > y0 {  // lower triangle, XY order: (0,0)->(1,0)->(1,1)
            offset_second_corner_x = 1u8;
            offset_second_corner_z = 0u8;
        } else { // upper triangle, YX order: (0,0)->(0,1)->(1,1)
            offset_second_corner_x = 0u8;
            offset_second_corner_z = 1u8;
        }
        let x1: f64 = x0 - offset_second_corner_x as f64 + G2;
        let y1: f64 = y0 - offset_second_corner_z as f64 + G2;
        let x2: f64 = x0 - 1.0f64 + 2.0f64 * G2;
        let y2: f64 = y0 - 1.0f64 + 2.0f64 * G2;
        let ii: u8 = (temperature_x & 0xFF) as u8;
        let jj: u8 = (temperature_z & 0xFF) as u8;
        let gi0: u8 = self.lookup(ii.wrapping_add(self.lookup(jj))) % 12u8;
        let gi1: u8 = self.lookup(ii.wrapping_add(offset_second_corner_x).wrapping_add(self.lookup(jj.wrapping_add(offset_second_corner_z)))) % 12u8;
        let gi2: u8 = self.lookup(ii.wrapping_add(1u8).wrapping_add(self.lookup(jj.wrapping_add(1u8)))) % 12u8;
        let t0: f64 = SimplexNoise::get_corner_noise3d(gi0, x0, y0, 0.0f64, 0.5f64);
        let t1: f64 = SimplexNoise::get_corner_noise3d(gi1, x1, y1, 0.0f64, 0.5f64);
        let t2: f64 = SimplexNoise::get_corner_noise3d(gi2, x2, y2, 0.0f64, 0.5f64);
        70.0f64 * (t0 + t1 + t2)
    }

    pub fn get_value_3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let key: u128 = (((x as u32) as u128) << 64 | ((y as u32) as u128) << 32 | ((z as u32) as u128)) as u128;
        let value: f64 = *self.cache3d.get(&key).unwrap_or(&std::f64::MAX);
        if value != std::f64::MAX {
            return value;
        }
        let value: f64 = self._get_value_3d(x, y, z);
        self.cache3d.insert(key, value);
        return value;
    }
    fn _get_value_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let skew_factor: f64 = (x + y + z) * F3; // F3 is 1/3
        // Skew the input space to determine which simplex cell we're in
        let i: i32 = (x + skew_factor).floor() as i32;
        let j: i32 = (y + skew_factor).floor() as i32;
        let k: i32 = (z + skew_factor).floor() as i32;
        let unskew_factor: f64 = (i + j + k) as f64 * G3; // G3 is 1/6
        // Unskew the cell origin back to (x,y,z) space
        let x0: f64 = (i) as f64 - unskew_factor;
        let y0: f64 = (j) as f64 - unskew_factor;
        let z0: f64 = (k) as f64 - unskew_factor;
        // The x,y,z distances from the cell origin
        let x0: f64 = x - x0;
        let y0: f64 = y - y0;
        let z0: f64 = z - z0;
        //For the 3D case, the simplex shape is a slightly irregular tetrahedron.
        // Determine which simplex we are in.
        let (i1, j1, k1): (u8, u8, u8); // Offsets for second corner of simplex in (i,j,k) coords
        let (i2, j2, k2): (u8, u8, u8); // Offsets for third corner of simplex in (i,j,k) coords
        if x0 >= y0 {
            if y0 >= z0 {
                i1 = 1u8;
                j1 = 0u8;
                k1 = 0u8;
                i2 = 1u8;
                j2 = 1u8;
                k2 = 0u8;
            } // X Y Z order
            else if x0 >= z0 {
                i1 = 1u8;
                j1 = 0u8;
                k1 = 0u8;
                i2 = 1u8;
                j2 = 0u8;
                k2 = 1u8;
            } // X Z Y order
            else {
                i1 = 0u8;
                j1 = 0u8;
                k1 = 1u8;
                i2 = 1u8;
                j2 = 0u8;
                k2 = 1u8;
            } // Z X Y order
        } else { // x0<y0
            if y0 < z0 {
                i1 = 0u8;
                j1 = 0u8;
                k1 = 1u8;
                i2 = 0u8;
                j2 = 1u8;
                k2 = 1u8;
            } // Z Y X order
            else if x0 < z0 {
                i1 = 0u8;
                j1 = 1u8;
                k1 = 0u8;
                i2 = 0u8;
                j2 = 1u8;
                k2 = 1u8;
            } // Y Z X order
            else {
                i1 = 0u8;
                j1 = 1u8;
                k1 = 0u8;
                i2 = 1u8;
                j2 = 1u8;
                k2 = 0u8;
            } // Y X Z order
        }

        let x1: f64 = x0 - i1 as f64 + G3; // Offsets for second corner in (x,y,z) coords
        let y1: f64 = y0 - j1 as f64 + G3;
        let z1: f64 = z0 - k1 as f64 + G3;
        let x2: f64 = x0 - i2 as f64 + F3; // Offsets for third corner in (x,y,z) coords
        let y2: f64 = y0 - j2 as f64 + F3;
        let z2: f64 = z0 - k2 as f64 + F3;
        let x3: f64 = x0 - 1.0f64 + 3.0f64 * G3; // Offsets for last corner in (x,y,z) coords
        let y3: f64 = y0 - 1.0f64 + 3.0f64 * G3;
        let z3: f64 = z0 - 1.0f64 + 3.0f64 * G3;
        let ii: u8 = (i & 0xff) as u8;
        let jj: u8 = (j & 0xff) as u8;
        let kk: u8 = (k & 0xff) as u8;
        let gi0: u8 = self.lookup(ii.wrapping_add(self.lookup(jj.wrapping_add(self.lookup(kk))))) % 12u8;
        let gi1: u8 = self.lookup(ii.wrapping_add(i1).wrapping_add(self.lookup(jj.wrapping_add(j1.wrapping_add(self.lookup(kk.wrapping_add(k1))))))) % 12u8;
        let gi2: u8 = self.lookup(ii.wrapping_add(i2).wrapping_add(self.lookup(jj.wrapping_add(j2.wrapping_add(self.lookup(kk.wrapping_add(k2))))))) % 12u8;
        let gi3: u8 = self.lookup(ii.wrapping_add(1).wrapping_add(self.lookup(jj.wrapping_add(1u8.wrapping_add(self.lookup(kk.wrapping_add(1u8))))))) % 12u8;

        // calculate the contribution of the 4 corners
        // should be 0.5 not 0.6 else the noise is not continuous on simplex boundaries but yeah mojang
        let t0: f64 = Self::get_corner_noise3d(gi0, x0, y0, z0, 0.6f64);
        let t1: f64 = Self::get_corner_noise3d(gi1, x1, y1, z1, 0.6f64);
        let t2: f64 = Self::get_corner_noise3d(gi2, x2, y2, z2, 0.6f64);
        let t3: f64 = Self::get_corner_noise3d(gi3, x3, y3, z3, 0.6f64);
        32.0f64 * (t0 + t1 + t2 + t3)
    }

    pub fn get_x0(&self) -> f64 {
        self.noise.x0
    }
    pub fn get_y0(&self) -> f64 {
        self.noise.y0
    }
    pub fn get_z0(&self) -> f64 {
        self.noise.z0
    }
    pub fn get_coordinates(&self) -> (f64, f64, f64) {
        (self.noise.x0, self.noise.y0, self.noise.z0)
    }
}

