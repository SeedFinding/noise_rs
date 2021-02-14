mod noise;
pub mod perlin_noise;
pub mod simplex_noise;
mod math;
mod double_perlin_noise;


#[cfg(test)]
mod simplex_test {
    use crate::simplex_noise::SimplexNoise;
    use java_random::Random;

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


#[cfg(test)]
mod perlin_test {
    use crate::perlin_noise::PerlinNoise;
    use java_random::Random;

    #[test]
    fn test_skip(){
        let random:Random=Random::with_seed(12);
        let perlin_noise:PerlinNoise=PerlinNoise::init(random,vec![4,5]);
        let value=perlin_noise.sample(0.0,0.0,0.0,0.0,0.0,false);
        println!("{}",value);
    }

}

pub fn create_range(low: i32, high: i32) -> Vec<i32> {
    (low..=high).collect()
}