//! Implements a naive two dimensional rectangular grid.

use rayon;

/// Index of a point in the grid.
#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct Idx(pub usize);

impl Idx {
    fn new(v: usize) -> Idx {
        Idx(v)
    }
}

/// Coordinates of a point in the grid.
#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct X(pub usize, pub usize);

/// Two-dimensional rectangular grid.
#[derive(Eq, Ord, PartialEq, PartialOrd, Copy, Clone, Debug)]
pub struct StructuredRectangular {
    pub x: usize,
    pub y: usize,
}

impl StructuredRectangular {
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.x * self.y
    }

    /// Iterator over all point indices in the grid
    #[inline(always)]
    pub fn ids(&self) -> impl Iterator<Item = Idx> {
        (0..self.size()).map(Idx)
    }

    /// Parallel iterator over all point indices in the grid
    #[inline(always)]
    pub fn par_ids(
        &self,
    ) -> rayon::iter::Map<rayon::range::Iter<usize>, fn(usize) -> Idx> {
        use rayon::iter::IntoParallelIterator;
        use rayon::iter::ParallelIterator;
        (0..self.size()).into_par_iter().map(Idx::new)
    }


    /// Returns the coordinates of a given point id
    #[inline(always)]
    pub fn x(&self, i: Idx) -> X {
        let x = {
            let mut cidx = i.0;
            while cidx > self.x - 1 {
                cidx -= self.x;
            }
            cidx
        };
        let y = i.0 / self.x;
        X(x, y)
    }

    /// Returns the id of the point at coordinates
    #[inline(always)]
    pub fn idx(&self, x: X) -> Idx {
        Idx(x.0 + self.x * x.1)
    }

    /// Returns the neighbor of the point `c` in direction `dir`
    #[inline(always)]
    pub fn neighbor<D: ::Distribution>(&self, c: Idx, dir: D) -> Idx {
        let X(x_i, y_i) = self.x(c);

        // handle periodic boundaries:
        let x_e = if x_i == (self.x - 1) { 0 } else { x_i + 1 };
        let x_w = if x_i == 0 { self.x - 1 } else { x_i - 1 };
        let y_n = if y_i == (self.y - 1) { 0 } else { y_i + 1 };
        let y_s = if y_i == 0 { self.y - 1 } else { y_i - 1 };
        use geometry::Direction::*;
        match dir.direction() {
            C => c,
            E => self.idx(X(x_e, y_i)),
            N => self.idx(X(x_i, y_n)),
            W => self.idx(X(x_w, y_i)),
            S => self.idx(X(x_i, y_s)),
            NE => self.idx(X(x_e, y_n)),
            NW => self.idx(X(x_w, y_n)),
            SW => self.idx(X(x_w, y_s)),
            SE => self.idx(X(x_e, y_s)),
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    fn test_grid() -> StructuredRectangular {
        StructuredRectangular { x: 4, y: 3 }
    }

    #[test]
    fn ids() {
        let g = test_grid();
        assert_eq!(g.size(), 12);
        assert_eq!(g.size(), g.ids().count());
        let mut c = 0;
        for i in g.ids() {
            assert_eq!(i, Idx(c));
            c += 1;
        }

        let mut c = 0;
        for j in 0..3 {
            for i in 0..4 {
                let x = X(i, j);
                let idx = Idx(c);
                println!("i: {}, j: {}, x: {:?}, idx: {:?}", i, j, x, idx);
                assert_eq!(g.x(idx), x);
                assert_eq!(g.idx(x), idx);



                c += 1;
            }
        }
    }
}
