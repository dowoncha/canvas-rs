use std::ops::{Add, Sub};
use types::{Point};

pub struct Rect<T> 
    where T: PartialOrd + Add<Output=T> + Sub<Output = T>
{
    left: T,
    top: T,
    right: T,
    bottom: T
}

impl<T> Rect<T> 
    where T: PartialOrd + Add<Output=T> + Sub<Output = T> + Copy
{
    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn set_ltrb(&mut self, left: T, top: T, right: T, bottom: T) {
        self.left = left;
        self.right = right;
        self.top = top;
        self.bottom = bottom;
    }

    pub fn set_xywh() {

    }

    pub fn set_wh() {

    }

    pub fn offset() {

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
}

impl Rect<f32> {
    // Order: top-left, top-right, bottom-left, bottom-right
    pub fn points(&self) -> (Point, Point, Point, Point) {
        (
            Point::new(self.x(), self.y()),
            Point::new(self.x() + self.width(),self.y()),
            Point::new(self.x(), self.y() + self.height()),
            Point::new(self.x() + self.width(), self.y() + self.height())
        )
    }
}