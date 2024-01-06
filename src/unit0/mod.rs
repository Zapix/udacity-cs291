
pub mod sample_1;
mod sample_2;

use std::rc::Rc;
use crate::common::unit_trait::UnitTrait;
use self::sample_1::Sample1;
use self::sample_2::Sample2;

pub fn get_units() -> Vec<Rc<Box<dyn UnitTrait>>> {
    vec![
        Rc::new(Box::new(Sample1::new())),
        Rc::new(Box::new(Sample2::new())),
    ]
}