use crate::geometry::point::{Point, Point2, Point3};

#[derive(Debug, Clone)]
pub struct Polygon(Box<[Point]>);

#[derive(Debug, Clone)]
pub struct Polygon2(Box<[Point2]>);

#[derive(Debug, Clone)]
pub struct Polygon3(Box<[Point3]>);
