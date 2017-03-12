//! A pixel packed into a u32
//! Formatted in ARGB

use std::ops::{Shr, BitAnd};

const GPIXEL_SHIFT_A: u32 = 24;
const GPIXEL_SHIFT_R: u32 = 16;
const GPIXEL_SHIFT_G: u32 = 8;
const GPIXEL_SHIFT_B: u32 = 0;

#[derive(PartialEq, Debug)]
pub struct Pixel(u32);

impl Pixel {
    pub fn new(a: u32, r: u32, g: u32, b: u32 ) -> Pixel {
        assert!(a <= 255u32);
        assert!(r <= a);
        assert!(g <= a);
        assert!(b <= a);

        let out = (a << GPIXEL_SHIFT_A) |
            (r << GPIXEL_SHIFT_R) |
            (g << GPIXEL_SHIFT_G) |
            (b << GPIXEL_SHIFT_B);

        Pixel(out)
    }
}


/**
 * Operations for a pixel
 */
impl Shr<Pixel> for Pixel {
    type Output = Self;

    fn shr(self, Pixel(rhs): Self) -> Pixel {
        let Pixel(lhs) = self;
        Pixel(lhs >> rhs)
    }
}

impl BitAnd for Pixel {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Pixel(self.0 & rhs.0)
    }
}

#[inline]
pub fn get_alpha(p: Pixel) -> u32 {
    ( p.0 >> GPIXEL_SHIFT_A ) & 0xFF
}
