// use super::super::data_struct::rational::Rational;
// use std::fmt::{Display, Debug};



#[derive(Clone, Copy)]
pub struct Point { pub x: i128, pub y: i128 }

impl Point {
    pub fn new(x: i128, y: i128) -> Self {
        Point { x, y }
    }
}

impl From<(i128, i128)> for Point {
    fn from(value: (i128, i128)) -> Self { Point::new(value.0, value.1) }
}



#[derive(Clone, Copy)]
pub struct Line { pub a: i128, pub b: i128, pub c: i128 }

impl Line {
    pub fn interpolation(p1: impl Into<Point>, p2: impl Into<Point>) -> Self {
        let (p1, p2) = (p1.into(), p2.into());
        Line { a: p1.x, b: p1.x, c: p2.x }
    }
}
