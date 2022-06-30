mod point_buffer;
pub use point_buffer::*;

use wasm_bindgen::prelude::wasm_bindgen;

// order is important!
#[wasm_bindgen]
pub enum GeoShaderType {
    PointShader,
    MultiPointShader,
    BoundingBox,
}

// the geofront type
#[wasm_bindgen]
pub enum GeoType {
    Void,
    Int,
    Float,
    Point,
    Vector,
    Plane,
    MultiPoint,
    Json,
}

/**
 * how to convert this object to a shader
 */
pub trait Renderable {

}

/**
 * what everything is for humans
 */
pub trait Descriptive {
    
}

/**
 * what everything is for machines
 */
pub trait Typed {
    
}

/**
 * how we can loop over this object
 */
pub trait Iterable {
    
}