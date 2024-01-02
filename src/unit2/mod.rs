mod triangle_mesh;

use std::rc::Rc;
use triangle_mesh::TriangleMesh;
use crate::common::traits::UnitTrait;

pub fn get_units() -> Vec<Rc<Box<dyn UnitTrait>>> {
    vec![
        Rc::new(Box::new(TriangleMesh::new())),
    ]
}