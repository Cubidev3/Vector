use std::ops;

pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub const ZERO: Vector2Int = Vector2Int { x: 0, y: 0 };
    pub const UP: Vector2Int = Vector2Int { x: 0, y: -1 };
    pub const DOWN: Vector2Int = Vector2Int { x: 0, y: 1 };
    pub const LEFT: Vector2Int = Vector2Int { x: -1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x: 1, y: 0 };

    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int { x, y }
    }

    pub fn squared_distance_to(&self, to: Vector2Int) -> i32 {
        (to.x - self.x).pow(2) - (to.y - self.y).pow(2)
    }

    pub fn distance_to(&self, to: Vector2Int) -> f32 {
        //Please note that this conversion is lossy and failure prone
        (self.squared_distance_to(to) as f32).sqrt()
    }

    pub fn squared_length(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2)
    }

    pub fn length(&self) -> f32 {
        //Please note that this conversion is lossy and failure prone
        (self.squared_length() as f32).sqrt()
    }

    pub fn sign(&self) -> Vector2Int {
        Vector2Int {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }

    pub fn abs(&self) -> Vector2Int {
        Vector2Int {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl ops::AddAssign<Vector2Int> for Vector2Int {
    fn add_assign(&mut self, rhs: Vector2Int) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::SubAssign<Vector2Int> for Vector2Int {
    fn sub_assign(&mut self, rhs: Vector2Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::MulAssign<i32> for Vector2Int {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::DivAssign<i32> for Vector2Int {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
