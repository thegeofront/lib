#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// all exports 
mod data;
mod maths;
mod misc;
mod multi;
mod raster;
mod types;
mod vector_0;
mod vector_1;
mod vector_2;
mod vector_3;
mod gfbind;

pub use crate::data::*;
pub use crate::maths::*;
pub use crate::misc::*;
pub use crate::multi::*;
pub use crate::raster::*;
pub use crate::types::*;
pub use crate::vector_0::*;
pub use crate::vector_1::*;
pub use crate::vector_2::*;
pub use crate::vector_3::*;
pub use crate::gfbind::*;

// the setup for wasm loading
mod utils;
#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start, skip_typescript)]
pub fn start() {
    utils::set_panic_hook();
}
