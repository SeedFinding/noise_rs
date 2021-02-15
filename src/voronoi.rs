#![allow(dead_code)]
use crate::math;
use std::collections::HashMap;

#[derive( Clone)]
pub struct Voronoi {
    cache: HashMap<u128,u128>,
    world_seed:i64,
}
pub fn next(world_seed: i64, salt: i64) -> i64 {
    return world_seed.wrapping_mul(world_seed.wrapping_mul(6364136223846793005i64).wrapping_add(1442695040888963407i64)).wrapping_add(salt);
}

impl Voronoi {
    pub fn new(world_seed: i64) -> Self {
        let cache: HashMap<u128,u128> = HashMap::new();
        Voronoi { cache,world_seed }
    }
    pub fn get_fuzzy_positions(&mut self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        let key: u128 = (((x as u32) as u128) << 64 | ((y as u32) as u128) << 32 | ((z as u32) as u128)) as u128;
        let mut value:u128=*self.cache.get(&key).unwrap_or(&u128::MAX);
        if value !=u128::MAX {
            let x:i32= (value >> 64 & 0xFFFF_FFFFu128) as i32;
            let y:i32= (value >> 32 & 0xFFFF_FFFFu128) as i32;
            let z:i32= (value & 0xFFFF_FFFFu128) as i32;
            return (x,y,z);
        }
        let (xx,yy,zz):(i32,i32,i32)=self._get_fuzzy_positions(x,y,z);
        value= ((xx as u128) << 64 | (yy as u128) << 32 | (zz as u128)) as u128;
        self.cache.insert(key,value);
        return (xx,yy,zz);
    }
    fn _get_fuzzy_positions(&mut self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        let moved_x: i32 = x - 2;
        let moved_y: i32 = y - 2;
        let moved_z: i32 = z - 2;
        let reduced_x: i32 = moved_x >> 2;
        let reduced_y: i32 = moved_y >> 2;
        let reduced_z: i32 = moved_z >> 2;
        let x_scaled: f64 = (moved_x & 3) as f64 / 4.0f64;
        let y_scaled: f64 = (moved_y & 3) as f64 / 4.0f64;
        let z_scaled: f64 = (moved_z & 3) as f64 / 4.0f64;
        let mut arrd: [f64; 8] = [0.0f64; 8];
        for cell in 0usize..8 {
            let high4: bool = (cell & 4) == 0;
            let high2: bool = (cell & 2) == 0;
            let high1: bool = (cell & 1) == 0;
            let xx: i32 = if high4 { reduced_x } else { reduced_x + 1 };
            let yy: i32 = if high2 { reduced_y } else { reduced_y + 1 };
            let zz: i32 = if high1 { reduced_z } else { reduced_z + 1 };
            let xx_scaled: f64 = if high4 { x_scaled } else { x_scaled - 1.0 };
            let yy_scaled: f64 = if high2 { y_scaled } else { y_scaled - 1.0 };
            let zz_scaled: f64 = if high1 { z_scaled } else { z_scaled - 1.0 };
            arrd[cell] = self.get_fiddled_distance(xx, yy, zz, xx_scaled, yy_scaled, zz_scaled);
        }
        let mut max_index: i32 = 0;
        let mut max: f64 = arrd[0];
        for cell in 1usize..8 {
            if !(max > arrd[cell]) { continue; }
            max_index = cell as i32;
            max = arrd[cell];
        }
        return (if (max_index & 4) == 0 { reduced_x } else { reduced_x + 1 }, if (max_index & 2) == 0 { reduced_y } else { reduced_y + 1 }, if (max_index & 1) == 0 { reduced_z } else { reduced_z + 1 });
    }

    fn get_fiddled_distance(&mut self, x: i32, y: i32, z: i32, x_scaled: f64, y_scaled: f64, z_scaled: f64) -> f64 {
        let mut fiddle: i64;
        // TODO remove constant part due to worldseed
        fiddle = self.world_seed.wrapping_mul(self.world_seed.wrapping_mul(6364136223846793005i64).wrapping_add(1442695040888963407i64)).wrapping_add(x as i64); // force inline
        fiddle = next(fiddle, y as i64);
        fiddle = next(fiddle, z as i64);
        fiddle = next(fiddle, x as i64);
        fiddle = next(fiddle, y as i64);
        fiddle = next(fiddle, z as i64);
        let x_offset: f64 = Self::get_fiddle(fiddle);
        fiddle = next(fiddle, self.world_seed);
        let y_offset: f64 = Self::get_fiddle(fiddle);
        fiddle = next(fiddle, self.world_seed);
        let z_offset: f64 = Self::get_fiddle(fiddle);
        return  Self::sqr(z_scaled + z_offset) +  Self::sqr(y_scaled + y_offset) + Self::sqr(x_scaled + x_offset);
    }

    fn get_fiddle(l: i64) -> f64 {
        return ((((math::floor_mod(l >> 24, 1024i64)) as i32) as f64 / 1024.0f64) - 0.5) * 0.9;
    }

    fn sqr(d: f64) -> f64 {
        d * d
    }

}

