pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}
