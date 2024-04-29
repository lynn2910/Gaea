pub mod border;

use std::ops::{Add, Sub};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type LandType = Uuid;

#[derive(Serialize, Deserialize, Debug, Default, PartialEq, PartialOrd)]
pub struct Point(f64, f64);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.x() + rhs.x(), self.y() + rhs.y()) 
    }
}
impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Self(x, y) }
    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
}
