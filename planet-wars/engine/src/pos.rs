use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: Self) -> f32 {
        let x2 = (other.x - self.x).powf(2.0);
        let y2 = (other.y - self.y).powf(2.0);
        (x2 + y2).sqrt()
    }

    pub fn direction_to(&self, other: Self) -> Self {
        other - *self
    }

    pub fn unit_direction_to(&self, other: Self) -> Self {
        let direction = self.direction_to(other);
        let distance = self.distance_to(other);
        Self::new(direction.x / distance, direction.y / distance)
    }

    pub fn distance_to_line_segment(&self, a: Self, b: Self) -> f32 {
        let ab = b - a;
        let pa = a - *self;

        let c = Pos::dot_product(ab, pa);
        if c > 0.0 {
            // Closest point is a
            return Pos::dot_product(pa, pa);
        }

        let bp = *self - b;
        if Pos::dot_product(ab, bp) > 0.0 {
            // Closest point is b
            return Pos::dot_product(bp, bp);
        }

        let closest = pa - ab * (c / Pos::dot_product(ab, ab));
        Pos::dot_product(closest, closest).sqrt()
    }
}

impl Pos {
    fn dot_product(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self::new(self.x + other.x, self.y + other.y);
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Pos {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Pos {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}
