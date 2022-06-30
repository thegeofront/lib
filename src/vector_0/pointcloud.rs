use core::panic;

use las::{self, Reader, Read, Version};

use std::{io::Cursor, convert::TryInto};

use serde::{Serialize, Deserialize};
use wasm_bindgen::{prelude::wasm_bindgen};

use crate::MultiPoint;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Pointcloud {
    reader: Reader,
}

#[wasm_bindgen]
impl Pointcloud {
    
    pub fn new_from_buffer(buffer: &[u8]) -> Self {
        let cursor = Cursor::new(buffer.to_owned());
        let reader = Reader::new(cursor).unwrap();
        let head = reader.header();
        
        println!("version: {}", head.version());
        println!("count: {} points.", head.number_of_points());
        println!("extends: {:?} points.", head.bounds());

        Self { reader }
    }

    pub fn version(&self) -> Vec<u8> { 
        let v = self.reader.header().version(); 
        vec![v.minor, v.major]
    }
    pub fn bounds(&self) -> Vec<f64> { 
        let b = self.reader.header().bounds();
        vec![b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z] 
    }

    pub fn to_array(&mut self) -> Vec<f64> {
        
        const POINT_SIZE: usize = 3;

        let head = self.reader.header();
        let num_points: usize = head.number_of_points().try_into().unwrap();        
        let mut vector = Vec::with_capacity(num_points * POINT_SIZE);

        for point in self.reader.points() {
            let p = point.unwrap();  
            vector.push(p.x);
            vector.push(p.y);
            vector.push(p.z);
        }

        vector
    }

    pub fn length_of_buffer(buffer: &[u8]) -> i32 {
        buffer.len() as i32
    }
}