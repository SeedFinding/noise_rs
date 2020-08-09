#![allow(dead_code)]
use java_random::Random;
use std::collections::HashMap;
use intmap::IntMap;
use crate::noise::Noise;

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
    pub fn init(mut random: Random,octaves:Vec<i32>) -> () {
        let start=-octaves.first().expect("Missing an element");
        let end=octaves.last().expect("Missing an element");
        let length=start+end+1;
        if length<1 {
           panic!("You need at least one octave")
        }
        let noise:Noise=Noise::init(random);
    }
}
