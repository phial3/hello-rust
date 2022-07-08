use euclid::Size2D;
use euclid::UnknownUnit;

pub struct Point {
    pub inner: Size2D<i32, UnknownUnit>,
}

impl Point {
    pub const fn new(d: i32) -> Self {
        Self {
            inner: Size2D::new(d, d),
        }
    }
}
