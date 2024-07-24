use std::f32::consts::PI;

use rand::Rng;
use wasm_bindgen::prelude::*;


fn random_layout(width: u8, height: u8, number: u8) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..number)
        .flat_map(|_| [rng.gen::<u8>() % width, rng.gen::<u8>() % height])
        .collect::<Vec<u8>>()
}

fn circle_layout(width: u8, height: u8, number: u8) -> Vec<u8> {
    let cx = width / 2;
    let cy = height / 2;
    let radius = std::cmp::min(cx, cy) as f32 * 0.8;

    let step = PI * 2.0 / number as f32;
    (0..number)
        .flat_map(|i| [
            (cx as f32 + (f32::sin(step * i as f32) * radius)) as u8,
            (cy as f32 + (f32::cos(step * i as f32) * radius)) as u8,
        ])
        .collect::<Vec<u8>>()
}

#[wasm_bindgen]
#[derive(Copy, Clone)]
pub enum LocationLayout {
    Random,
    Circle,
}

impl LocationLayout {
    fn layout(&self, width: u8, height: u8, number: u8) -> Vec<u8> {
        match self {
            Self::Random => random_layout(width, height, number),
            Self::Circle => circle_layout(width, height, number),
        }
    }
}

pub struct Map {
    width: u8,
    height: u8,
    locations: Vec<u8>,
}

impl Map {
    pub fn new(width: u8, height: u8, number: u8, layout: LocationLayout) -> Self {
        Self {
            width,
            height,
            locations: layout.layout(width, height, number),
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn locations(&self) -> &Vec<u8> {
        &self.locations
    }

    pub fn coords(&self, location_idx: usize) -> [u8; 2] {
        [self.locations[location_idx * 2], self.locations[location_idx * 2 + 1]]
    }
}
