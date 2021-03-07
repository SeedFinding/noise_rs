mod noise;
pub mod perlin_noise;
pub mod simplex_noise;
pub mod math;
pub mod double_perlin_noise;
pub mod voronoi;

pub fn create_range(low: i32, high: i32) -> Vec<i32> {
    (low..=high).collect()
}