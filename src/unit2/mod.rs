mod triangle_mesh;
mod vertex_order;
mod polygon_creation;
mod polygon_location;
mod polygon_radius;

use std::rc::Rc;
use triangle_mesh::TriangleMesh;
use vertex_order::VertexOrder;
use polygon_creation::PolygonCreation;
use polygon_location::PolygonLocation;
use polygon_radius::PolygonRadius;

use crate::common::unit_trait::UnitTrait;

pub fn get_units() -> Vec<Rc<Box<dyn UnitTrait>>> {
    vec![
        Rc::new(Box::new(TriangleMesh::new())),
        Rc::new(Box::new(VertexOrder::new())),
        Rc::new(Box::new(PolygonCreation::new())),
        Rc::new(Box::new(PolygonLocation::new())),
        Rc::new(Box::new(PolygonRadius::new())),
    ]
}