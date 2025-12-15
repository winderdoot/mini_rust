use crate::game_logic::province::{self, ProvinceType};
use bevy::{prelude::*};
use noise::{NoiseFn, Simplex, Seedable};

const COOL_CONST: u32 = 213769420;

pub trait FloatExt1 {
    fn lerp_to(self, old_min: Self, old_max: Self, new_min: Self, new_max: Self) -> Self;
}

impl FloatExt1 for f32 {
    fn lerp_to(self, old_min: Self, old_max: Self, new_min: Self, new_max: Self) -> Self {
        let t = (self - old_min) / (old_max - old_min);
        new_min + t * (new_max - new_min)
    }
}

pub struct ProvinceGenerator {
    height: Simplex, /* Simplex noise generates values in range [-1, 1] */
    humidity: Simplex,
    min_height: f32,
    max_height: f32,
    water_level: f32
}

impl ProvinceGenerator {
    pub fn new(seed: u32, min_height: f32, max_height: f32) -> Self {
        let sea_level = min_height.lerp(max_height, 0.35);
        ProvinceGenerator {
            height: Simplex::new(seed),
            humidity: Simplex::new(seed ^ COOL_CONST),
            min_height,
            max_height,
            water_level: sea_level
        }
    }

    /// Remaping the \[-1, 1\] simplex value to \[0, 1\] for ease of use
    fn sample(noise: &Simplex, at: &Vec2, freq: f32) -> f32 {
        noise.get([(at.x * freq) as f64, (at.y * freq) as f64]) as f32
    }

    /// Calculates province type and province altitude
    pub fn get_province(&self, p: &Vec2) -> (ProvinceType, f32) {
        let h = self.get_height(&p);
        // let humid = self.humidity.get([p.x as f64, p.y as f64]);
        
        if h < self.water_level {
            return (ProvinceType::Water, self.water_level - 0.05f32)
        }

        (ProvinceType::Woods, h)
    }

    pub fn get_height(&self, p: &Vec2) -> f32 {
        let layers = [(0.02, 0.1), (0.05, 0.5), (0.1, 0.1), (0.2, 0.05), (0.5, 0.01)];
        let (sum, w) = layers
            .map(|(freq, weight)| {
                (Self::sample(&self.height, p, freq).lerp_to(-1.0, 1.0, 0.0, 1.0) * weight, weight)
            })
            .iter()
            .fold((0.0, 0.0), |(sum, sum_w), (s, w)| (sum + s, sum_w + w));
        let sample = sum / w;

        // let sample = Self::sample(&self.height, p, 0.05)
        //     .lerp_to(-1.0, 1.0, 0.0, 1.0);

        self.min_height + sample * self.max_height
    }
}