use java_random::Random;
use crate::math::{modf, lerp3, grad, smooth_step};
use crate::math;
use std::fmt;

#[derive(Clone)]
pub struct Noise {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub permutations: [u8; 256],
}

fn unsize<T>(x: &[T]) -> &[T] { x }

impl fmt::Debug for Noise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Noise")
            .field("x0", &self.x0)
            .field("y0", &self.y0)
            .field("z0", &self.z0)
            .field("permutations", &unsize(&self.permutations))
            .finish()
    }
}

#[cfg(test)]
mod noise_test {
    use super::*;

    #[test]
    fn test_gen_1() {
        let noise = Noise::new(&mut Random::with_seed(1));
        let value = noise.get_noise_value(0f64, 0f64, 0f64, 0f64, 0f64);
        assert_eq!(value, 0.10709059654197703f64);
    }

    #[test]
    fn test_gen_1million() {
        let noise = Noise::new(&mut Random::with_seed(1));
        let mut score: f64 = 0.0;
        let bound = 100;
        for x in 0..bound {
            for y in 0..bound {
                for z in 0..bound {
                    score += noise.get_noise_value(x as f64, y as f64, z as f64, 0f64, 0f64);
                }
            }
        }
        assert_eq!(score, 5.106111820344766f64);
    }
}

impl Noise {
    pub fn new(random: &mut Random) -> Noise {
        let x0: f64 = random.next_double() * 256.0;
        let y0: f64 = random.next_double() * 256.0;
        let z0: f64 = random.next_double() * 256.0;
        let mut permutations: [u8; 256] = [0; 256];
        for index in 0u8..=255 {
            permutations[index as usize] = index;
        }
        for index in 0u8..=255 {
            let random_index: u8 = random.next_int_n(256i32 - index as i32) as u8;
            let temp: u8 = permutations[(random_index + index) as usize];
            permutations[(random_index + index) as usize] = permutations[index as usize];
            permutations[index as usize] = temp;
        }
        Noise { x0, y0, z0, permutations }
    }

    pub fn get_noise_value(&self, x: f64, y: f64, z: f64, y_amplification: f64, min_y: f64) -> f64 {
        let offset_x: f64 = x + self.x0;
        let offset_y: f64 = y + self.y0;
        let offset_z: f64 = z + self.z0;
        let (integer_x, fractional_x) = modf(offset_x);
        let (integer_y, fractional_y) = modf(offset_y);
        let (integer_z, fractional_z) = modf(offset_z);
        let mut clamp_y: f64 = 0f64;
        if y_amplification != 0.0 {
            clamp_y = (math::min(min_y, fractional_y) / y_amplification).floor() * y_amplification;
        }
        self.sample_and_lerp(integer_x as i32,
                             integer_y as i32,
                             integer_z as i32,
                             fractional_x,
                             fractional_y - clamp_y,
                             fractional_z,
                             smooth_step(fractional_x),
                             smooth_step(fractional_y),
                             smooth_step(fractional_z))
    }

    pub fn sample_and_lerp(&self, int_x: i32, int_y: i32, int_z: i32, frac_x: f64, frac_y: f64, frac_z: f64, smooth_x: f64, smooth_y: f64, smooth_z: f64) -> f64 {
        let px_y = (self.lookup(int_x) as i32) + int_y;
        let px1_y = (self.lookup(int_x + 1) as i32) + int_y;

        let ppx_y_z = (self.lookup(px_y) as i32) + int_z;
        let ppx1_y_z = (self.lookup(px1_y) as i32) + int_z;

        let ppx_y1_z = (self.lookup(px_y + 1) as i32) + int_z;
        let ppx1_y1_z = (self.lookup(px1_y + 1) as i32) + int_z;

        let x1: f64 = grad(self.lookup(ppx_y_z), frac_x, frac_y, frac_z);
        let x2: f64 = grad(self.lookup(ppx1_y_z), frac_x - 1.0f64, frac_y, frac_z);
        let x3: f64 = grad(self.lookup(ppx_y1_z), frac_x, frac_y - 1.0f64, frac_z);
        let x4: f64 = grad(self.lookup(ppx1_y1_z), frac_x - 1.0f64, frac_y - 1.0f64, frac_z);
        let x5: f64 = grad(self.lookup(ppx_y_z + 1), frac_x, frac_y, frac_z - 1.0f64);
        let x6: f64 = grad(self.lookup(ppx1_y_z + 1), frac_x - 1.0f64, frac_y, frac_z - 1.0f64);
        let x7: f64 = grad(self.lookup(ppx_y1_z + 1), frac_x, frac_y - 1.0f64, frac_z - 1.0f64);
        let x8: f64 = grad(self.lookup(ppx1_y1_z + 1), frac_x - 1.0f64, frac_y - 1.0f64, frac_z - 1.0f64);

        lerp3(smooth_x, smooth_y, smooth_z, x1, x2, x3, x4, x5, x6, x7, x8)
    }

    pub fn lookup(&self, index: i32) -> u8 {
        self.permutations[(index & 0xff) as usize]
    }
}