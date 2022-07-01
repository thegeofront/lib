use serde_json::json;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{BoundingBox, GeoShaderType, Pointcloud, PointBuffer, MultiPointBuffer};

#[wasm_bindgen]
pub struct MultiPoint {
    data: Vec<f64>,
}

#[wasm_bindgen]
impl MultiPoint {

    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn new_from_array(data: Vec<f64>) -> Self {
        // TODO SET BOUNDING BOX 
        Self { data }
    }

    pub fn new_from_pointcloud(pc: &mut Pointcloud) -> Self {
        Self::new_from_array(pc.to_array())
    }

    pub fn push_pt(&mut self, pt: crate::Point) {
        self.data.push(pt.p.x);
        self.data.push(pt.p.y);
        self.data.push(pt.p.z);
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

    pub fn calc_bounding_box(&self) -> BoundingBox {
        BoundingBox::new_default()
    }

}
    // impl Renderable for Point 
#[wasm_bindgen]
impl MultiPoint {

    pub fn gf_has_trait_renderable() -> bool {
        true
    }

    pub fn gf_get_shader_type() -> GeoShaderType {
        GeoShaderType::MultiPointShader
    }

    pub fn gf_get_bounding_box(&self) -> JsValue {
        JsValue::from_serde(&self.calc_bounding_box()).unwrap()
    }

    pub fn gf_get_buffers(&self) -> JsValue {
        let buffer = MultiPointBuffer {
            verts: self.data.clone()
        };
        JsValue::from_serde(&buffer).unwrap()
    }
}
