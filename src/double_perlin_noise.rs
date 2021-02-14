#![allow(dead_code)]
use crate::perlin_noise::PerlinNoise;
use java_random::Random;

#[derive(Clone)]
pub struct DoublePerlinNoise {
    amplitude: f64,
    first_perlin: PerlinNoise,
    second_perlin: PerlinNoise,
}

impl DoublePerlinNoise {
    pub fn new(random: Random, octaves: Vec<i32>) -> Self {
        let min_octave = octaves.iter().min().unwrap_or(&0);
        let max_octave = octaves.iter().max().unwrap_or(&0);
        DoublePerlinNoise {
            amplitude: 0.16666666666666666f64 / Self::create_amplitude(max_octave - min_octave),
            first_perlin: PerlinNoise::init(random, octaves.clone()),
            second_perlin: PerlinNoise::init(random, octaves.clone()),
        }
    }

    fn create_amplitude(length: i32) -> f64 {
        0.1f64 * (1.0f64 + 1.0f64 / ((length + 1) as f64))
    }

    pub fn sample(self,x:f64,y:f64,z:f64)->f64{
        let skewed_x = x * 1.0181268882175227f64;
        let skewed_y = y * 1.0181268882175227f64;
        let skewed_z = z * 1.0181268882175227f64;
        return (self.first_perlin.sample_default(x, y, z) + self.second_perlin.sample_default(skewed_x, skewed_y, skewed_z)) * self.amplitude;
    }
}