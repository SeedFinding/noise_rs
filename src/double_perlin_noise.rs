#![allow(dead_code)]

use crate::perlin_noise::PerlinNoise;
use java_random::Random;

#[derive(Clone, Debug)]
pub struct DoublePerlinNoise {
    amplitude: f64,
    first_perlin: PerlinNoise,
    second_perlin: PerlinNoise,
}

#[cfg(test)]
mod double_perlin_test {
    use super::*;
    use crate::create_range;

    #[test]
    fn test_gen1() {
        let double_perlin = DoublePerlinNoise::new(&mut Random::with_seed(1), create_range(1, 2));
        let value = double_perlin.sample(0f64, 0f64, 0f64);
        assert_eq!(value, -0.273983041873796f64)
    }

    #[test]
    fn test_gen2() {
        let double_perlin = DoublePerlinNoise::new(&mut Random::with_seed(1), create_range(-7, -6));
        let value = double_perlin.sample(25f64, 0f64, 24f64);
        assert_eq!(value, 0.07304369034293899f64)
    }

    #[test]
    fn test_gen_1million() {
        let noise = DoublePerlinNoise::new(&mut Random::with_seed(1), create_range(1, 2));
        let mut score: f64 = 0.0;
        let bound = 100;
        for x in 0..bound {
            for y in 0..bound {
                for z in 0..bound {
                    score += noise.sample(x as f64, y as f64, z as f64);
                }
            }
        }
        assert_eq!(score, 32.885536183861234);
    }
}

impl DoublePerlinNoise {
    pub fn new(random: &mut Random, octaves: Vec<i32>) -> Self {
        let min_octave = octaves.iter().min().unwrap_or(&0);
        let max_octave = octaves.iter().max().unwrap_or(&0);
        DoublePerlinNoise {
            amplitude: 0.16666666666666666f64 / Self::create_amplitude(max_octave - min_octave),
            first_perlin: PerlinNoise::new(random, octaves.clone()),
            second_perlin: PerlinNoise::new(random, octaves.clone()),
        }
    }

    fn create_amplitude(length: i32) -> f64 {
        0.1f64 * (1.0f64 + 1.0f64 / ((length + 1) as f64))
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let skewed_x = x * 1.0181268882175227f64;
        let skewed_y = y * 1.0181268882175227f64;
        let skewed_z = z * 1.0181268882175227f64;
        return (self.first_perlin.sample_default(x, y, z) + self.second_perlin.sample_default(skewed_x, skewed_y, skewed_z)) * self.amplitude;
    }
}