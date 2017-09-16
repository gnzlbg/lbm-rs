use std;
use num;
use geometry::Direction;
use traits;

#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
#[repr(usize)]
pub enum D2Q9 {
    C = 0,
    E = 1,
    N = 2,
    W = 3,
    S = 4,
    NE = 5,
    NW = 6,
    SW = 7,
    SE = 8,
}

impl D2Q9 {
    #[inline(always)]
    fn new(v: usize) -> D2Q9 {
        assert!(v < 9);
        unsafe { std::mem::transmute(v) }
    }
}

type Iter = std::iter::Map<std::ops::Range<usize>, fn(usize) -> D2Q9>;

impl traits::Distribution for D2Q9 {
    type Storage = [num; 9];
    type AllIterator = Iter;
    #[inline(always)]
    fn c_squ() -> num {
        1. / 3.
    }
    #[inline(always)]
    fn size() -> usize {
        9
    }
    #[inline(always)]
    fn all() -> Self::AllIterator {
        (0..Self::size()).map(D2Q9::new)
    }
    #[inline(always)]
    fn opposite(&self) -> D2Q9 {
        use self::D2Q9::*;
        match *self {
            C => C,
            E => W,
            N => S,
            W => E,
            S => N,
            NE => SW,
            NW => SE,
            SW => NE,
            SE => NW,
        }
    }
    #[inline(always)]
    fn value(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn constant(&self) -> num {
        use self::D2Q9::*;
        match *self {
            C => 4. / 9.,
            E | N | W | S => 1. / 9.,
            NE | NW | SW | SE => 1. / 36.,
        }
    }
    #[inline(always)]
    fn direction(&self) -> Direction {
        use self::D2Q9::*;
        match *self {
            C => Direction::C,
            E => Direction::E,
            N => Direction::N,
            W => Direction::W,
            S => Direction::S,
            NE => Direction::NE,
            NW => Direction::NW,
            SW => Direction::SW,
            SE => Direction::SE,
        }
    }

    #[inline(always)]
    fn from_direction(d: Direction) -> Option<Self> {
        use self::D2Q9::*;
        Some(match d {
                 Direction::C => C,
                 Direction::E => E,
                 Direction::N => N,
                 Direction::W => W,
                 Direction::S => S,
                 Direction::NE => NE,
                 Direction::NW => NW,
                 Direction::SW => SW,
                 Direction::SE => SE,
             })
    }
    #[inline(always)]
    fn center() -> Self {
        D2Q9::C
    }
}

impl traits::DirectDistribution for D2Q9 {
    type DirectIterator = Iter;
    #[inline(always)]
    fn direct() -> Self::DirectIterator {
        (1..5).map(D2Q9::new)
    }
}

impl traits::DiagonalDistribution for D2Q9 {
    type DiagonalIterator = Iter;
    #[inline(always)]
    fn diagonal() -> Self::DiagonalIterator {
        (5..9).map(D2Q9::new)
    }
}

#[cfg(test)]
mod tests {
    use super::D2Q9;
    #[test]
    fn opposite() {
        assert_eq!(D2Q9::C.opposite(), D2Q9::C);
        assert_eq!(D2Q9::E.opposite(), D2Q9::W);
        assert_eq!(D2Q9::N.opposite(), D2Q9::S);
        assert_eq!(D2Q9::W.opposite(), D2Q9::E);
        assert_eq!(D2Q9::S.opposite(), D2Q9::N);
        assert_eq!(D2Q9::NE.opposite(), D2Q9::SW);
        assert_eq!(D2Q9::NW.opposite(), D2Q9::SE);
        assert_eq!(D2Q9::SW.opposite(), D2Q9::NE);
        assert_eq!(D2Q9::SE.opposite(), D2Q9::NW);
    }
    #[test]
    fn velocities() {
        assert_eq!(D2Q9::C.direction(), [0, 0]);
        assert_eq!(D2Q9::E.direction(), [1, 0]);
        assert_eq!(D2Q9::N.direction(), [0, 1]);
        assert_eq!(D2Q9::W.direction(), [-1, 0]);
        assert_eq!(D2Q9::S.direction(), [0, -1]);
        assert_eq!(D2Q9::NE.direction(), [1, 1]);
        assert_eq!(D2Q9::NW.direction(), [-1, 1]);
        assert_eq!(D2Q9::SW.direction(), [-1, -1]);
        assert_eq!(D2Q9::SE.direction(), [1, -1]);
    }
    #[test]
    fn values() {
        let mut c = 0;
        for i in D2Q9::all() {
            let v = i.value();
            assert_eq!(v, c);
            let j = D2Q9::new(v);
            assert_eq!(i, j);
            c += 1;
        }
        assert_eq!(D2Q9::size(), D2Q9::all().count());
    }


}
