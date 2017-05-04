use std::ops::{Add, Sub};
use types::{PointF};
use nalgebra::Point2;
use std::fmt::Debug;

pub type RectPoints = (PointF, PointF);

#[derive(Debug)]
pub struct Rect<T> 
{
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T
}

impl From<RectPoints> for Rect<f32> {
    fn from(points: RectPoints) -> Rect<f32> {
        let tl = points.0;
        let br = points.1;

        Rect::xywh(tl.x, tl.y, br.x, br.y)
    }
}

impl<T: 'static> Rect<T> 
    where T: PartialOrd + Add<Output=T> + Sub<Output = T> + Copy + Debug
{
    pub fn xywh(x: T, y: T, w: T, h: T) -> Rect<T> {
        Rect {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h
        }
    }

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn set_ltrb(&mut self, left: T, top: T, right: T, bottom: T) {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
    }

    pub fn set_xywh(&mut self) {

    }

    pub fn set_wh(&mut self) {

    }

    pub fn offset(&mut self) {

    }

    pub fn x(&self) -> T {
        self.left
    }
    
    pub fn y(&self) -> T {
        self.top
    }

    pub fn left(&self) -> T {
        self.left
    }

    pub fn top(&self) -> T {
        self.top
    }

    pub fn right(&self) -> T {
        self.right
    }

    pub fn bottom(&self) -> T {
        self.bottom
    }

    pub fn width(&self) -> T {
        self.right - self.left
    }

    pub fn height(&self) -> T {
        self.bottom - self.top
    }

    pub fn points(&self) -> (Point2<T>, Point2<T>) {
        (
            Point2::new(self.x(), self.y()),
            Point2::new(self.x() + self.width(), self.y() + self.height())
        )
    }

    pub fn contains(&self, point: &Point2<T>) -> bool {
        self.left <= point.x && point.x <= self.right &&
            self.top <= point.y && point.y <= self.bottom
    }
}