use std::cell::Cell;
use nalgebra::{Matrix3, zero};
use image::{RgbaImage, Rgba, Pixel};
use types::Color;
use types::{RectF, RectI, Mat3f};
use shader::{Shader, PixelShader};
use util::color_to_pixel;

pub struct Canvas {
    // bitmap: Cell<RgbaImage>,
    bitmap: RgbaImage,
    ctm: Mat3f,
    matrix_stack: Vec<Mat3f>
}

impl Canvas {
    pub fn new(bitmap: RgbaImage) -> Canvas {
        Canvas {
            bitmap: bitmap, // Cell::new(bitmap)
            ctm: Mat3f::identity(),      // Current transformation matrix, usually the top of the stack
            matrix_stack: Vec::new()
        }
    }

    pub fn clear(&mut self, color: &Color) {
        // Convert given color into a pixel
        let clear_color = color_to_pixel(color);

        // Iterate through bitmap and set to given color
        for (x, y, pixel) in self.bitmap.enumerate_pixels_mut() {
            *pixel = clear_color;
        }
    }

    pub fn fill_rect(&mut self, rect: &RectF, color: &Color) {
        // Assert rect is not empty
        let pixel = color_to_pixel(color);

        // 1. Create a color shader
        let pixel_shader = Box::new(PixelShader::new(&pixel));
        
        self.shade_rect(rect, &*pixel_shader);
    }   

    pub fn shade_rect(&mut self, rect: &RectF, shader: &Shader) {
        // 1. Convert rectangle into points

        // 2. Apply ctm to points

        // 3. If transformed points are not a rect, draw a polygon

        // 4. Convert back to a rectangle

        // 5. Check rectangle is not empty and clip edges from bitmap

        // self.shade_device_rect(transformed_rect, shader);
    }

    pub fn shade_device_rect(&mut self, rect: &RectI, shader: &Shader) {
        // 1. Set shader context
        
        // 2. Create row
        // let sub_image = SubImage::new(self.bitmap, rect.left(), rect.top(), rect.width(), self.height());

            // shader.shade_row()

            // Blend rows
    }
}

fn blend(src: Rgba<u8>, dst: Rgba<u8>) -> Rgba<u8> {
    // If the alpha is 255 return src
    if src.data[3] == 0xFF {
        return src;
    }

    Rgba::from_channels(0, 0, 0, 0)
}
