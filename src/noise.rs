use java_random::Random;

#[derive(Clone, Copy)]
pub struct Noise {
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub permutations: [u8; 256],
}

impl Noise {
    #[cold]
    pub fn new(x0: f64, y0: f64, z0: f64, permutations: [u8; 256]) -> Self {
        Noise { x0, y0, z0, permutations }
    }
    pub fn init(mut random: Random) -> Noise {
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
}