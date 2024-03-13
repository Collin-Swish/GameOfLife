use std::ops::{Add, Mul};
use std::fmt;
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Cell{
    pub x: i64,
    pub y: i64,
}

impl Add for Cell {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Mul for Cell {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self {
        Self { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}



impl Mul<Cell> for i64 {
    type Output = Cell;
    fn mul(self, rhs: Cell) -> Self::Output {
        Cell {
            x: rhs.x * self,
            y: rhs.y * self,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Cell {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(x: i64, y: i64) -> Self {
        Cell { x: x, y: y }
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn mul(&self, other: &Cell) -> Cell {
        return self.clone() * other.clone();
    }
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn add(&self, other: &Cell) -> Cell {
        return self.clone() + other.clone();
    }
}