use num;
use grid;
use super::Geometry;

pub struct Rectangle {
    x_center: [num; 2],
    lengths: [num; 2],
}

impl Rectangle {
    pub fn new(x_center: [num; 2], lengths: [num; 2]) -> Self {
        Self { x_center, lengths }
    }
}

impl Geometry for Rectangle {
    #[inline(always)]
    fn contains(&self, _: grid::X) -> bool {
        unimplemented!()
    }
}
