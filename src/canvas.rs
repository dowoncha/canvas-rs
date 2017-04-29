use image::{RgbaImage, Rgba, Pixel};
use types::Color;
use types::{RectF, RectI, Mat3f};
use shader::{Shader, PixelShader, BitmapShader};
use util::color_to_pixel;

// TODO: Eventually convert RgbaImage into GenericImage trait
// to allow for all image and pixel types

pub struct Canvas<'a> {
    // bitmap: Cell<RgbaImage>,
    bitmap: &'a mut RgbaImage,
    ctm: Mat3f,
    matrix_stack: Vec<Mat3f>
}

impl<'a> Canvas<'a> {
    pub fn new(bitmap: &'a mut RgbaImage) -> Canvas<'a> {
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

    pub fn fill_rect(&mut self, dst: &RectF, color: &Color) {
        // Assert rect is not empty
        let pixel = color_to_pixel(color);

        // 1. Create a color shader
        let pixel_shader = PixelShader::new(&pixel);
        
        self.shade_rect(dst, &pixel_shader);
    }   

    pub fn fill_bitmap_rect(&mut self, src: &RgbaImage, dst: &RectF ) {
        // 1. Create matrix of conversion from src to dest rect
        let transform = Mat3f::identity();

        // 2. Use conversion matrix to create bitmap shader
        let shader = BitmapShader::new(src, transform);

        self.shade_rect(dst, &shader);
    }

    pub fn shade_rect(&mut self, rect: &RectF, shader: &Shader) {
        // 1. Convert rectangle into points
        let (tl, tr, bl, br) = rect.points();

        // 2. Apply ctm to points
        // let tl = self.ctm * 

        // 3. If transformed points are not a rect, draw a polygon

        // 4. Convert back to a rectangle

        // 5. Check rectangle is not empty and clip edges from bitmap

        // self.shade_device_rect(transformed_rect, shader);
    }

    /// ------ Current Transformation Matrix Functions

    /// Current Transformation Matrix functions
    pub fn save(&mut self) {
        self.matrix_stack.push(self.ctm);
    }

    /// Restores the saved matrix into the stack
    pub fn restore(&mut self) {
        if let Some(ctm) = self.matrix_stack.pop() {
            self.ctm = ctm;
        }
    }

    /// Concats the input matrix onto the ctm
    pub fn concat(&mut self, input: Mat3f) {
        self.ctm *= input;
    }

    fn shade_device_rect(&mut self, rect: &RectI, shader: &Shader) {
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
