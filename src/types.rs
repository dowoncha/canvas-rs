use nalgebra::{Vector2, Matrix3, Point2};
use rect::Rect;
use image::{Rgba};

pub type GPixel = Rgba<u8>;
pub type Mat3f = Matrix3<f32>;
pub type Color = Rgba<f32>;
pub type RectF = Rect<f32>;
pub type RectI = Rect<i32>;
pub type PointF = Point2<f32>;
pub type PointU = Point2<u32>;
pub type Vec2f = Vector2<f32>;