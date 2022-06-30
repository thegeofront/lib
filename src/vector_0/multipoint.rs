use wasm_bindgen::prelude::wasm_bindgen;

use crate::{Pointcloud, Point};

#[wasm_bindgen]
pub struct MultiPoint {
    data: Vec<f64>
}

#[wasm_bindgen]
impl MultiPoint {

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn new_from_array(data: Vec<f64>) -> Self {
        Self { data }
    }

    pub fn new_from_pointcloud(pc: &mut Pointcloud) {
        Self::new_from_array(pc.to_array());
    }

    pub fn add_pt(&mut self, pt: Point) {
        self.data.push(pt.x);
        self.data.push(pt.y);
        self.data.push(pt.z);
    }

    pub fn add_num(&mut self, x: f64, y: f64, z: f64) {
        self.data.push(x);
        self.data.push(y);
        self.data.push(z);
    }

    /**
     * Get a copy of the data array within
     */
    pub fn get_data(&self) -> Vec<f64> {
        self.data.clone()
    }

}