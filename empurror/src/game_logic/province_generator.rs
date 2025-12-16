use crate::game_logic::province::{self, ProvinceType};
use bevy::{prelude::*};
use hexx::*;
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

const DESERT_HUMID: f32 = 0.36;
const PLAIN_HUMID: f32 = 0.6;
const BLACKSOIL_HUMID: f32 = 0.6;

pub struct ProvinceGenerator {
    height: Simplex, /* Simplex noise generates values in range [-1, 1] */
    humidity: Simplex,
    min_height: f32,
    max_height: f32,
    water_level: f32,
    hill_level: f32,
    mountain_level: f32,
}

impl ProvinceGenerator {
    pub fn new(seed: u32, min_height: f32, max_height: f32) -> Self {
        let water_level = min_height.lerp(max_height, 0.35);
        let hill_level = min_height.lerp(max_height, 0.65);
        let mountain_level = min_height.lerp(max_height, 0.71);
        ProvinceGenerator {
            height: Simplex::new(seed),
            humidity: Simplex::new(seed ^ COOL_CONST),
            min_height,
            max_height,
            water_level,
            hill_level,
            mountain_level
        }
    }

    fn sample(noise: &Simplex, at: &Vec2, freq: f32) -> f32 {
        noise.get([(at.x * freq) as f64, (at.y * freq) as f64]) as f32
    }

    /// Calculates province type and province altitude
    pub fn get_province(&self, p: &Vec2) -> (ProvinceType, f32) {
        let h = self.get_height(&p);
        
        if h < self.water_level {
            return (ProvinceType::Water, self.water_level - 0.05f32)
        }
        if h > self.mountain_level {
            return (ProvinceType::Mountains, h)
        }
        
        let humid = self.get_humidity(&p);

        if h < self.hill_level {
            match humid {
                ..DESERT_HUMID => return (ProvinceType::Desert, h),
                ..PLAIN_HUMID => return (ProvinceType::Plains, h),
                _ => return (ProvinceType::Woods, h)
            }
        }

        return (ProvinceType::Hills, h)
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

        self.min_height + sample * self.max_height
    }

    fn get_humidity(&self, p: &Vec2) -> f32 {
        let layers = [(0.02, 0.3), (0.05, 0.5), (0.1, 0.1), (0.2, 0.05), (0.5, 0.05)];
        let (sum, w) = layers
            .map(|(freq, weight)| {
                (Self::sample(&self.humidity, p, freq).lerp_to(-1.0, 1.0, 0.0, 1.0) * weight, weight)
            })
            .iter()
            .fold((0.0, 0.0), |(sum, sum_w), (s, w)| (sum + s, sum_w + w));
        let sample = sum / w;
        
        sample
    }

    fn count_water_neighbors(&self, hex: &Hex, layout: &HexLayout) -> f32 {
        hex
            .all_neighbors()
            .iter()
            .map(|hex| self.get_height(&layout.hex_to_world_pos(*hex)))
            .filter(|h| *h < self.water_level)
            .count() as f32
    } 

    /* Theoretically I could implement this using iterators and do it on a hex to hex basis, but I am leaving myself the option
     * of using a vec to make a more complicated algorithm */
    pub fn generate(&self, tiles: impl ExactSizeIterator<Item = Hex>, layout: &HexLayout) -> Vec<(Hex, Vec3, ProvinceType)> {
        let mut tiles: Vec<_> = tiles
            .map(|hex| {
                let pos = layout.hex_to_world_pos(hex);
                let (province, h) = self.get_province(&pos);

                (hex, Vec3::new(pos.x, h, pos.y), province)
            })
            .collect();
        
            for (hex, pos, province) in &mut tiles {
                if let ProvinceType::Water = province {
                    continue;
                }
                let water_amount = (self.count_water_neighbors(hex, layout) + 8.0 * self.get_humidity(&layout.hex_to_world_pos(*hex))) / 10.0;
                if water_amount > BLACKSOIL_HUMID {
                    *province = ProvinceType::BlackSoil
                }
            }

        tiles
    }


}