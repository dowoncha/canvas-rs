use image::{Pixel, Rgba};
use types::{Color};
use std::cmp;

// TODO: Implement as a FROM trait
#[inline]
pub fn color_to_pixel(color: &Color) -> Rgba<u8> {
    // pin to unit
    let (fR, fG, fB, fA) = color.channels4();

    let fA = fA * 255.9999;
    let uA = fA as u8;
    let uR = (fR * fA) as u8;
    let uG = (fG * fG) as u8;
    let uB = (fB * fB) as u8;

    return Rgba::from_channels(uR, uG, uB, uA);
}

#[inline]
pub fn pin_to_unit(x: f32) -> f32 {
    0.0f32.max(1.0f32.min(x))
}

#[inline]
pub fn mul_div_255_round(a: u8, b: u8) -> u8 {
    let mut product: u32 = (a * b) as u32 * 65793;
    product += 1 << 23;
    ( product >> 24 ) as u8
}

#[inline]
pub fn clamp<T: Ord>(min: T, value: T, max: T) -> T {
    cmp::max(min, cmp::min(value, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_to_unit() {
        assert_eq!(0.0, pin_to_unit(-5.0));
        assert_eq!(1.0, pin_to_unit(5.0));
        assert_eq!(0.5, pin_to_unit(0.5));
    }

    #[test]
    fn test_mul_div() {

    }
}
