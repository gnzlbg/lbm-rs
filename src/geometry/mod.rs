use num;
use grid;

mod circle;
mod plane;
mod rectangle;

pub use self::circle::Circle;
pub use self::plane::Plane;
pub use self::rectangle::Rectangle;

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
        [num::from(x), num::from(y)]
    }
}

pub trait Geometry {
    #[inline(always)]
    fn contains(&self, x: grid::X) -> bool;
}
