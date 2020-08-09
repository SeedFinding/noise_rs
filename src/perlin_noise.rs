#![allow(dead_code)]
use java_random::Random;
use std::collections::HashMap;
use intmap::IntMap;
use crate::noise::Noise;

#[derive(Clone)]
pub struct PerlinNoise {
    noise_octaves:Vec<Noise>,
    cache2d: IntMap<f64>,
    cache3d: HashMap<u128, f64>,
}

impl PerlinNoise {
    pub fn init(mut random: Random,octaves:Vec<i32>) -> () {
        let start=-*octaves.first().expect("Missing an element");
        let end=*octaves.last().expect("Missing an element");
        let length=start+end+1;
        if length<1 {
           panic!("You need at least one octave")
        }
        let noise:Noise=Noise::init(random);
        let noise_octaves:Vec<Noise>=vec![noise,length];
        for i in end + 1..length {
            if i>=0 && octaves.contains(&(end - i)){
                noise_octaves[i]=Noise::init(random);
                continue;
            }
            random.skip_with_LCG(262);
        }
        if (end>0){
            let noise_seed:i64=Self::noise(0.0, 0.0, 0.0, 0.0, 0.0) * 9.223372036854776E18;
            random=Random::with_seed(noise_seed as u64);
            for i in range(end-1,-1).rev(){
                if i<length && octaves.contains(end-i){
                    noise_octaves[i]=Noise::init(random);
                    continue;
                }
                random.skip_with_LCG(262);
            }
        }
        let highestFreqInputFactor = Math.pow(2.0, end);
        let highestFreqValueFactor = 1.0 / (Math.pow(2.0, length) - 1.0);
    }
}
