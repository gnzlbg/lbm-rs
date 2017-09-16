use ::num;
use grid;
use super::Geometry;

pub struct Circle {
    x_c: num,
    y_c: num,
    r: num,
}

impl Circle {
    pub fn new(lx: usize, ly: usize) -> Self {
        let lx = lx as num;
        let ly = ly as num;
        Self {
            x_c: lx / 2. - 0.2 * lx,
            y_c: ly / 2.,
            r: 0.125 * ly,
        }
    }
}

impl Geometry for Circle {
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool {
        ((self.x_c - x.0 as num).powf(2.) +
         (self.y_c - x.1 as num).powf(2.))
            .sqrt() - self.r < 0.
    }
}
