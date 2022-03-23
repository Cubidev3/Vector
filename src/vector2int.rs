use super::vector3int::Vector3Int;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Vector2Int {
    pub x: i32,
    pub y: i32,
}

impl Vector2Int {
    pub const ZERO: Vector2Int = Vector2Int { x: 0, y: 0 };
    pub const ONE: Vector2Int = Vector2Int { x: 1, y: 1 };
    pub const UP: Vector2Int = Vector2Int { x: 0, y: 1 };
    pub const DOWN: Vector2Int = Vector2Int { x: 0, y: -1 };
    pub const LEFT: Vector2Int = Vector2Int { x: -1, y: 0 };
    pub const RIGHT: Vector2Int = Vector2Int { x: 1, y: 0 };

    pub fn new(x: i32, y: i32) -> Vector2Int {
        Vector2Int { x, y }
    }

    pub fn squared_distance_to(&self, to: Vector2Int) -> i32 {
        (to.x - self.x).pow(2) + (to.y - self.y).pow(2)
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

    pub fn oposite(&self) -> Vector2Int {
        Vector2Int {
            x: self.x * -1,
            y: self.y * -1,
        }
    }
}

impl ops::Add for Vector2Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Vector2Int {
        Vector2Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<Vector3Int> for Vector2Int {
    type Output = Vector3Int;

    fn add(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: rhs.z,
        }
    }
}

impl ops::AddAssign<Vector2Int> for Vector2Int {
    fn add_assign(&mut self, rhs: Vector2Int) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector2Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Vector2Int {
        Vector2Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Sub<Vector3Int> for Vector2Int {
    type Output = Vector3Int;

    fn sub(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: 0 - rhs.z,
        }
    }
}

impl ops::SubAssign<Vector2Int> for Vector2Int {
    fn sub_assign(&mut self, rhs: Vector2Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Vector2Int {
    type Output = Self;

    fn mul(self, rhs: Self) -> Vector2Int {
        Vector2Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl ops::Mul<Vector2Int> for i32 {
    type Output = Vector2Int;

    fn mul(self, rhs: Vector2Int) -> Vector2Int {
        Vector2Int {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl ops::Mul<i32> for Vector2Int {
    type Output = Self;

    fn mul(self, rhs: i32) -> Vector2Int {
        Vector2Int {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl ops::Mul<Vector3Int> for Vector2Int {
    type Output = Vector3Int;

    fn mul(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: 0,
        }
    }
}

impl ops::MulAssign<i32> for Vector2Int {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl ops::MulAssign<Vector2Int> for Vector2Int {
    fn mul_assign(&mut self, rhs: Vector2Int) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl ops::Div<i32> for Vector2Int {
    type Output = Self;

    fn div(self, rhs: i32) -> Vector2Int {
        Vector2Int {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl ops::Div for Vector2Int {
    type Output = Self;

    fn div(self, rhs: Vector2Int) -> Vector2Int {
        Vector2Int {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl ops::DivAssign<i32> for Vector2Int {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl ops::DivAssign<Vector2Int> for Vector2Int {
    fn div_assign(&mut self, rhs: Vector2Int) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
