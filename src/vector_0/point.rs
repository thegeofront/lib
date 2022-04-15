use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

#[wasm_bindgen]
impl Point {
    
    pub fn default() -> Point {
        Point { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x , y, z}
    }

    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn mover(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}