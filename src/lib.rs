#![allow(dead_code)]

use wasm_bindgen::prelude::*;

// all exports 
mod data;
mod basic;
mod misc;
mod raster;
mod geo_point;
mod geo_line;
mod geo_srf;
mod geo_solid;
mod gfbind;

pub use crate::data::*;
pub use crate::basic::*;
pub use crate::misc::*;
pub use crate::raster::*;
pub use crate::geo_point::*;
pub use crate::geo_line::*;
pub use crate::geo_srf::*;
pub use crate::geo_solid::*;
pub use crate::gfbind::*;

// the setup for wasm loading
mod wasm_util;

#[wasm_bindgen]
extern "C" {
    // fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start, skip_typescript)]
pub fn start() {
    wasm_util::set_panic_hook();
}
