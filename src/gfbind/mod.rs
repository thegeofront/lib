mod shader_buffers;
pub use shader_buffers::*;

use wasm_bindgen::prelude::wasm_bindgen;

// order is important!


// the geofront type
#[wasm_bindgen]
pub enum GeoType {

    
    Void,
    Int,
    Float,
    
    Point,
    Vector,
    Plane,
    Transform,


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