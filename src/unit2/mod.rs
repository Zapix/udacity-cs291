mod triangle_mesh;
mod vertex_order;
mod polygon_creation;

use std::rc::Rc;
use triangle_mesh::TriangleMesh;
use vertex_order::VertexOrder;
use polygon_creation::PolygonCreation;
use crate::common::unit_trait::UnitTrait;

pub fn get_units() -> Vec<Rc<Box<dyn UnitTrait>>> {
    vec![
        Rc::new(Box::new(TriangleMesh::new())),
        Rc::new(Box::new(VertexOrder::new())),
        Rc::new(Box::new(PolygonCreation::new())),
    ]
}