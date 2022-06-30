
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PointBuffer {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}