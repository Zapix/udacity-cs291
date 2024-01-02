use crate::common::geometry::point::Point;
use crate::common::geometry::face3::Face3;

pub struct Geometry {
    pub verticies: Vec<Point>,
    pub faces: Vec<Face3>,
}

impl Geometry {
    pub fn new() -> Self {
        Self {
            verticies: vec![],
            faces: vec![],
        }
    }
}