use nalgebra::inverse;
use image::{Rgba, RgbaImage, Pixel};
use types::{PointU, Mat3f, GPixel};

pub trait Shader {
    fn set_context(&mut self, ctx: Mat3f) -> bool {
        true
    }

    // TODO: Row needs enumeration of x, y
    fn shade_row(&self, start: &PointU, row: &mut [Rgba<u8>]);

    fn sample(&self, point: &PointU) -> GPixel {
        GPixel::from_channels(0, 0, 0, 0)
    }
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
    fn shade_row(&self, start: &PointU, row: &mut [Rgba<u8>]) {
        for pixel in row {
            *pixel = self.src;
        }
    }

    fn sample(&self, point: &PointU) -> GPixel {
        self.src
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
    fn set_context(&mut self, ctx: Mat3f) -> bool {
        self.local = ctx;
        // self.inverse =

        true
    }

    fn shade_row(&self, start: &PointU, row: &mut [Rgba<u8>]) {
    }
}