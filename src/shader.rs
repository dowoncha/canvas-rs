use nalgebra::inverse;
use image::{Rgba, RgbaImage};
use types::Mat3f;

pub trait Shader {
    // fn set_context(ctm: Mat) -> bool;

    // TODO: Row needs enumeration of x, y
    fn shade_row(&self, row: &mut [Rgba<u8>]);
}

pub struct PixelShader {
    src: Rgba<u8>
}

impl PixelShader {
    pub fn new(color: &Rgba<u8>) -> PixelShader {
        PixelShader {
            src: color.clone()
        }
    } 
}

impl Shader for PixelShader {
    fn shade_row(&self, row: &mut [Rgba<u8>]) {
        for pixel in row {
            *pixel = self.src;
        }
    }
}

pub struct BitmapShader<'a> {
    src: &'a RgbaImage,
    local: Mat3f,
    inverse: Mat3f
}

impl<'a> BitmapShader<'a> {
    pub fn new(src: &'a RgbaImage, local: Mat3f) -> BitmapShader<'a> {
        // FIXME
        // let inverse = inverse(&local);

        BitmapShader {
            src: src,
            local: local,
            inverse: local
        }
    }
}

impl<'a> Shader for BitmapShader<'a> {
    fn shade_row(&self, row: &mut [Rgba<u8>]) {
        for pixel in row {

        }
    }
}