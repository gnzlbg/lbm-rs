use super::num;
use grid;

#[repr(usize)]
pub enum Direction {
    C,
    E,
    N,
    W,
    S,
    NE,
    NW,
    SW,
    SE,
}

impl Direction {
    #[inline(always)]
    pub fn array(&self) -> [i8; 2] {
        use self::Direction::*;
        match *self {
            C => [0, 0],
            E => [1, 0],
            N => [0, 1],
            W => [-1, 0],
            S => [0, -1],
            NE => [1, 1],
            NW => [-1, 1],
            SW => [-1, -1],
            SE => [1, -1],
        }
    }
    #[inline(always)]
    pub fn num_array(&self) -> [num; 2] {
        let [x, y] = self.array();
        [x as num, y as num]
    }
}


pub trait Geometry {
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool;
}

pub struct Cylinder {
    x_c: num,
    y_c: num,
    r: num,
}

impl Cylinder {
    pub fn new(lx: usize, ly: usize) -> Cylinder {
        let lx = lx as num;
        let ly = ly as num;
        Cylinder {
            x_c: lx / 2. - 0.2 * lx,
            y_c: ly / 2.,
            r: 0.125 * ly,
        }
    }
}

impl Geometry for Cylinder {
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool {
        ((self.x_c - x.0 as num).powf(2.) +
         (self.y_c - x.1 as num).powf(2.))
                .sqrt() - self.r < 0.
    }
}


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
