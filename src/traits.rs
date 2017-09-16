use num;
use geometry;
use io::vtk;
use grid;

/// TODO: move traits to appropriate modules

pub trait Distribution: Sized + Copy + Sync + Send {
    type Storage: AsRef<[num]> + AsMut<[num]> + Default;
    type AllIterator: Iterator<Item = Self>;
    #[inline(always)]
    fn all() -> Self::AllIterator;
    #[inline(always)]
    fn c_squ() -> num;
    #[inline(always)]
    fn direction(&self) -> geometry::Direction;
    #[inline(always)]
    fn from_direction(geometry::Direction) -> Option<Self>;
    #[inline(always)]
    fn constant(&self) -> num;
    #[inline(always)]
    fn size() -> usize;
    #[inline(always)]
    fn value(&self) -> usize;
    #[inline(always)]
    fn center() -> Self;
    #[inline(always)]
    fn opposite(&self) -> Self;
}

pub type DistributionStorage<D> = <D as Distribution>::Storage;

pub trait DirectDistribution: Distribution {
    type DirectIterator: Iterator<Item = Self>;
    #[inline(always)]
    fn direct() -> Self::DirectIterator;
}

pub trait DiagonalDistribution: Distribution {
    type DiagonalIterator: Iterator<Item = Self>;
    #[inline(always)]
    fn diagonal() -> Self::DiagonalIterator;
}


pub trait Collision<D: Distribution>: Copy + Sync + Send {
    #[inline(always)]
    fn collision<H, IH>(&self, f_hlp: &H, idx_h: IH) -> D::Storage
    where
        IH: Fn(&H, D) -> num;
}


pub trait Physics: Copy + Sync + Send {
    type Distribution: Distribution;
    #[inline(always)]
    fn collision<FH, IFH>(
        &self,
        f_h: &FH,
        idx_f_h: IFH,
    ) -> DistributionStorage<Self::Distribution>
    where
        IFH: Fn(&FH, Self::Distribution) -> num;
    #[inline(always)]
    fn integral<F: Fn(Self::Distribution) -> num>(_: F) -> num {
        0.0
    }

    fn write<O, F>(
        &self,
        vtk_writer: vtk::CellDataWriter,
        _: O,
        _: F,
    ) -> vtk::CellDataWriter
    where
        F: Fn(grid::Idx, Self::Distribution) -> num,
        O: Fn(grid::Idx) -> bool,
    {
        vtk_writer
    }
}
