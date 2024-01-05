mod triangle_mesh;
mod vertex_order;

use std::rc::Rc;
use triangle_mesh::TriangleMesh;
use vertex_order::VertexOrder;
use crate::common::traits::UnitTrait;

pub fn get_units() -> Vec<Rc<Box<dyn UnitTrait>>> {
    vec![
        Rc::new(Box::new(TriangleMesh::new())),
        Rc::new(Box::new(VertexOrder::new())),
    ]
}