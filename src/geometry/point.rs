use ::num;
use std;

pub trait AmbientDimension {
    fn ambient_dimension(&self) -> usize;
}

pub trait ObjectDimension {
    fn object_dimension(&self) -> usize;
}

pub trait Point: AmbientDimension + ObjectDimension + std::ops::Index<num> {

}

struct Point2D {
    x: [num; 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_tests() {
    }
}
