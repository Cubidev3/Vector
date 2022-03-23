use super::vector2int::Vector2Int;
use std::ops;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Vector3Int {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3Int {
    pub const ZERO: Vector3Int = Vector3Int { x: 0, y: 0, z: 0 };
    pub const ONE: Vector3Int = Vector3Int { x: 1, y: 1, z: 1 };
    pub const UP: Vector3Int = Vector3Int { x: 0, y: 1, z: 0 };
    pub const DOWN: Vector3Int = Vector3Int { x: 0, y: -1, z: 0 };
    pub const LEFT: Vector3Int = Vector3Int { x: -1, y: 0, z: 0 };
    pub const RIGHT: Vector3Int = Vector3Int { x: 1, y: 0, z: 0 };
    pub const FORWARD: Vector3Int = Vector3Int { x: 0, y: 0, z: 1 };
    pub const BACK: Vector3Int = Vector3Int { x: 0, y: 0, z: -1 };

    pub fn new(x: i32, y: i32, z: i32) -> Vector3Int {
        Vector3Int { x, y, z }
    }

    pub fn from(from: Vector2Int) -> Vector3Int {
        Vector3Int {
            x: from.x,
            y: from.y,
            z: 0,
        }
    }

    pub fn squared_distance_to(&self, to: Vector3Int) -> i32 {
        (to.x - self.x).pow(2) + (to.y - self.y).pow(2) + (to.z - self.z).pow(2)
    }

    pub fn distance_to(&self, to: Vector3Int) -> f32 {
        (self.squared_distance_to(to) as f32).sqrt()
    }

    pub fn squared_length(&self) -> i32 {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    pub fn length(&self) -> f32 {
        (self.squared_length() as f32).sqrt()
    }

    pub fn sign(&self) -> Vector3Int {
        Vector3Int {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    pub fn abs(&self) -> Vector3Int {
        Vector3Int {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn oposite(&self) -> Vector3Int {
        Vector3Int {
            x: self.x * -1,
            y: self.y * -1,
            z: self.z * -1,
        }
    }
}

impl ops::Add for Vector3Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Vector3Int {
        Vector3Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Add<Vector2Int> for Vector3Int {
    type Output = Self;

    fn add(self, rhs: Vector2Int) -> Vector3Int {
        Vector3Int {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z,
        }
    }
}

impl ops::AddAssign<Vector3Int> for Vector3Int {
    fn add_assign(&mut self, rhs: Vector3Int) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::AddAssign<Vector2Int> for Vector3Int {
    fn add_assign(&mut self, rhs: Vector2Int) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector3Int {
    type Output = Self;

    fn sub(self, rhs: Self) -> Vector3Int {
        Vector3Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Sub<Vector2Int> for Vector3Int {
    type Output = Self;

    fn sub(self, rhs: Vector2Int) -> Vector3Int {
        Vector3Int {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z,
        }
    }
}

impl ops::SubAssign<Vector3Int> for Vector3Int {
    fn sub_assign(&mut self, rhs: Vector3Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::SubAssign<Vector2Int> for Vector3Int {
    fn sub_assign(&mut self, rhs: Vector2Int) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul for Vector3Int {
    type Output = Self;

    fn mul(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<Vector2Int> for Vector3Int {
    type Output = Self;

    fn mul(self, rhs: Vector2Int) -> Vector3Int {
        Vector3Int {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: 0,
        }
    }
}

impl ops::Mul<i32> for Vector3Int {
    type Output = Self;

    fn mul(self, rhs: i32) -> Vector3Int {
        Vector3Int {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vector3Int> for i32 {
    type Output = Vector3Int;

    fn mul(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl ops::MulAssign<i32> for Vector3Int {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::MulAssign<Vector3Int> for Vector3Int {
    fn mul_assign(&mut self, rhs: Vector3Int) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<Vector2Int> for Vector3Int {
    fn mul_assign(&mut self, rhs: Vector2Int) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z = 0;
    }
}

impl ops::Div<Vector3Int> for Vector3Int {
    type Output = Self;

    fn div(self, rhs: Vector3Int) -> Vector3Int {
        Vector3Int {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl ops::Div<i32> for Vector3Int {
    type Output = Self;

    fn div(self, rhs: i32) -> Vector3Int {
        Vector3Int {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::DivAssign<i32> for Vector3Int {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl ops::DivAssign<Vector3Int> for Vector3Int {
    fn div_assign(&mut self, rhs: Vector3Int) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}
