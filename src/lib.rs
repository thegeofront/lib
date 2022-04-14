#![allow(dead_code)]

use wasm_bindgen::prelude::*;

mod vector_0;

pub use crate::vector_0::{Point, Vector};

mod utils;



#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
}
