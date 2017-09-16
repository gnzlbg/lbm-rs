//! I/O traits
use ::num;

/// Point trait TODO: move this to geometry
pub trait Point {
    fn size() -> usize;
    fn get(&self, i: usize) -> num;
}

/// Primitive types
pub trait Primitive {
    fn name() -> &'static str;
}

/// Cell types
pub enum CellType {
    Rectangle,
}

/// Cell geometry
pub trait CellGeometry {
    type Point: Point;
    type PointIterator: Iterator<Item = Self::Point>;
    fn cell_type(&self) -> CellType;
    fn cell_points(&self) -> Self::PointIterator;
}


pub trait CellDataWriter<CellIdx> {
    fn write_scalar<T: Primitive>(&mut self, name: &str, f: Fn(CellIdx) -> T);
}

pub trait Serializable {
    type CellIndex;
    type CellGeometry: CellGeometry;
    type CellIndexIterator: Iterator<Item = Self::CellIndex>;
    fn cells(&self) -> Self::CellIndexIterator;
    fn geometry(&self, Self::CellIndex) -> Self::CellGeometry;
    fn cell_data<T: CellDataWriter<Self::CellIndex>>(&self, &mut T);
}
