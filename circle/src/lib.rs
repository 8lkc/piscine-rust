use std::f64::consts::PI;

#[derive(Debug)]
pub struct Point {pub x: f64, pub y: f64}

impl Point {
    pub fn distance(&self, other: &Point) -> f64 {((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()}
}

#[derive(Debug)]
pub struct Circle {pub center: Point, pub radius: f64}

impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Self {
        Self {center: Point { x, y }, radius}
    }
    pub fn diameter(&self) -> f64 {self.radius * 2.0}
    pub fn area(&self) -> f64 {PI * self.radius.powi(2)}
    pub fn intersect(&self, other: &Circle) -> bool {
        let distance = self.center.distance(&other.center); distance <= (self.radius + other.radius)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let circle = Circle::new(500.0, 500.0, 150.0);
        let circle1 = Circle {center: Point { x: 80.0, y: 115.0 }, radius: 30.0};
        let point_a = Point { x: 1.0, y: 1.0 };
        let point_b = Point { x: 0.0, y: 0.0 };

        assert_eq!(circle.area(), 70685.83470577035);
        assert_eq!(circle.diameter(), 300.0); assert_eq!(circle1.diameter(), 60.0);
        assert_eq!(circle.intersect(&circle1), false);
        assert_eq!(point_a.distance(&point_b), 1.4142135623730951)
    }
}
