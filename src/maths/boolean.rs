use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub struct Boolean {}

// These functions have magic names, and we need them on the 
// javascript side to make the flowchart work and operate smoothly
#[wasm_bindgen] 
impl Boolean {
    pub fn and(a: bool, b: bool) {
        
    }

    pub fn or() {

    }

    pub fn not() {

    }

}
