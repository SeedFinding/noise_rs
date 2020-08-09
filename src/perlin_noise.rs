#![allow(dead_code)]
use java_random::Random;
use std::collections::HashMap;
use intmap::IntMap;

#[derive(Clone)]
pub struct PerlinNoise {
    x0: f64,
    y0: f64,
    z0: f64,
    permutations: [u8; 256],
    cache2d: IntMap<f64>,
    cache3d: HashMap<u128, f64>,
}

impl PerlinNoise {
    pub fn init(mut random: Random) -> () {}
}
