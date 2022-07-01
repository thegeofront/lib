use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::wasm_bindgen;

use glam::DVec3 as vec3;

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct BoundingBox {
    a: vec3,
    b: vec3,
}

#[wasm_bindgen]
impl BoundingBox {

    pub fn new(x1: f64, y1: f64, z1: f64, x2: f64, y2: f64, z2: f64) -> Self {
        Self {
            a: vec3::new(x1, y1, z1), 
            b: vec3::new(x2, y2, z2)
        }
    }

    pub fn new_default() -> Self {
        Self {
            a: vec3::NAN.clone(), 
            b: vec3::NAN.clone()
        }
    }
}

