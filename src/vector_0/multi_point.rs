use serde::{Serialize, Deserialize};
use serde_json::json;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use crate::{GeoShaderType, PointBuffer, GeoType};

use crate::Point;

#[wasm_bindgen(inspectable)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MultiPoint {
    data: Vec<f64>
}

#[wasm_bindgen]
impl MultiPoint {
    
}
