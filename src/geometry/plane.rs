use grid;
use super::Geometry;

pub struct Plane {
    n: (isize, isize),
    x: (usize, usize),
}

impl Plane {
    pub fn new(n: (isize, isize), x: (usize, usize)) -> Plane {
        Plane { n: n, x: x }
    }
}

impl Geometry for Plane {
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool {
        match (self.n.0, self.n.1, x.0, x.1) {
            (1, 0, x, _) => if x > self.x.0 { false } else { true },
            (0, 1, _, y) => if y > self.x.1 { false } else { true },
            (0, -1, _, y) => if y < self.x.1 { false } else { true },
            _ => unimplemented!(),
        }
    }
}
