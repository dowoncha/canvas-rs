use image::{RgbaImage, Rgba, Pixel, SubImage, GenericImage};
use types::{GPixel, Vec2f, PointF, PointU, RectF, RectI, Mat3f, Color};
use shader::{Shader, PixelShader, BitmapShader};
use util::{color_to_pixel, mul_div_255_round, clamp};
use edge::Edge;
use rect::Rect;

use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::borrow::BorrowMut;
use std::ops::DerefMut;

// TODO: Eventually convert RgbaImage into GenericImage trait
// to allow for all image and pixel types

type Edges = Vec<Edge>;

pub enum FillStyle<'a> {
    Rgba(Rgba<u8>),
    Bitmap(&'a RgbaImage)
}

pub enum StrokeStyle {
    Solid
}

pub struct Canvas<'a> {
    // bitmap: Cell<RgbaImage>,
    bitmap: RefCell<&'a mut RgbaImage>,
    ctm: Mat3f,
    matrix_stack: Vec<Mat3f>,
    fill_style: FillStyle<'a>
    // pub stroke_style: StrokeStyle
}

impl<'a> Canvas<'a> {
    pub fn new(bitmap: &'a mut RgbaImage) -> Canvas<'a> {
        Canvas {
            // bitmap: Rc::new(RefCell::new(bitmap)),
            bitmap: RefCell::new(bitmap),
            ctm: Mat3f::identity(),      // Current transformation matrix, usually the top of the stack
            matrix_stack: Vec::new(),
            fill_style: FillStyle::Rgba(GPixel::from_channels(0, 0, 0, 0)),
            // stroke_style: StrokeStyle::Solid
        }
    }

    // ----- Context manipulation
    pub fn set_fill_style(&mut self, style: FillStyle<'a>) {
        self.fill_style = style;
    }

    pub fn translate(&self, x: i32, y: i32) {
        self.ctm.append_translation(&Vec2f::new(x as f32, y as f32));
    }

    pub fn rotate(&self, angle: f32) {

    }

    pub fn scale(&self, x: f32, y: f32) {
        self.ctm.append_nonuniform_scaling(&Vec2f::new(x, y));
    }

    pub fn set_transform(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) {
        let ctm = Mat3f::new(a, b, c, d, e, f, 0.0, 0.0, 1.0);
        self.ctm = ctm;
    }

    pub fn reset_transform(&mut self) {
        self.ctm = Mat3f::identity();
    }

    // ----- Public rect drawing functions

    /// Clear the specified rectangular area, making it fully transparent
    pub fn clear_rect(&self, x: u32, y: u32, width: u32, height: u32) {
        let clear = GPixel::from_channels(0, 0, 0, 0);
        
        let mut borrow = self.bitmap.borrow_mut();

        let mut sub_image = borrow.sub_image(x, y, width, height);

        for (x, y, pixel) in sub_image.pixels_mut() {
            *pixel = clear;
        }
    }

    pub fn fill_rect(&self, x: u32, y: u32, width: u32, height: u32) {
        assert!( width > 0, height > 0);
        
        let dst = Rect::xywh(x, y, width, height);

        let mut shader: Box<Shader> = match self.fill_style {
            FillStyle::Rgba(color) => {
                // 1. Create a color shader
                Box::new(PixelShader::new(&color))
            },
            FillStyle::Bitmap(image) => {
                // 2. Use conversion matrix to create bitmap shader
                Box::new(BitmapShader::new(image, Mat3f::identity()))
            },
            _ => {
                // If Fill style fails, shade black
                Box::new(PixelShader::new(&GPixel::from_channels(0, 0, 0, 255)))
            }
        };

        self.shade_rect(&dst, &mut *shader);
    }   

    // ------ Public image drawing functions
    pub fn draw_image(&self, image: &RgbaImage, x: u32, y: u32) {
        
    }

    /// ------ Current Transformation Matrix Functions

    /// Save the state of current transformation matrix
    /// Includes:
    ///     Transforms applied so far
    ///     Attributes
    pub fn save(&mut self) {
        self.matrix_stack.push(self.ctm);
    }

    /// Restores the saved matrix into the stack
    pub fn restore(&mut self) {
        if let Some(ctm) = self.matrix_stack.pop() {
            self.ctm = ctm;
        }
    }

    // ------ Private Device coordinate drawing functions

    // Convert rectangle into device coordinates
    fn shade_rect(&self, rect: &Rect<u32>, shader: &mut Shader) {
        // 1. Convert rectangle into points
        // Iterate over the points and transform them by ctm AKA Device mode
        let (tl, br) = rect.points();

        // let transform = |point: Point<u32>| {
        //     let x = self.ctm[0] * point.x + self.ctm[1] * point.y + self.ctm[2];
        //     let y = self.ctm[3] * point.x + self.ctm[4] * point.y + self.ctm[5];
        //     Point::new(x, y)
        // };

        // 3. If transformed points are not a rect, draw a polygon
        // Convert to edges
      
        // 4. Convert back to a rectangle

        // 5. Check rectangle is not empty and clip edges from bitmap

        self.shade_device_rect(rect, shader);
    }

    fn shade_device_rect(&self, rect: &Rect<u32>, shader: &mut Shader) {
        // println!("Shading rect in device mode: {:?}", rect);
        // 1. Set shader context
        shader.set_context(self.ctm);

        // 2. Create row
        let mut borrow = self.bitmap.borrow_mut();
        
        let mut sub_image = borrow.sub_image(
            rect.x(), 
            rect.y(), 
            rect.width(), 
            rect.height()
        );

        for (x, y, pixel) in sub_image.pixels_mut() {
            let src = shader.sample(&PointU::new(x, y));
            *pixel = blend(src, *pixel);
        }
    }

    fn shade_device_polygon(&self, edges: &Edges, shader: &Shader) {
        unimplemented!()
    }
}

fn blend_row(src: &[GPixel], dst: &[GPixel]) -> Vec<GPixel> {
    assert!(src.len() == dst.len());

    src.iter().zip(dst.iter())
        .map(|(src_p, dst_p)| blend(*src_p, *dst_p))
        .collect()
}

/// Blend 2 Pixels according to alpha values
fn blend(src: GPixel, dst: GPixel) -> GPixel {
    let src_a = src.data[3];

    // If the alpha is 255 return src
    if src_a == 0xFF {
        return src;
    }

    if src_a == 0 {
        return dst;
    }

    // Calculat alpha blend amount
    let src_blend: u8 = 0xFF - src_a;

    let alpha = src_a + mul_div_255_round(src_blend, dst.data[3]);

    // Calculate rgb as blend of the alpha
    let mut red = src.data[0] + mul_div_255_round(src_blend, dst.data[0]);
    red = clamp(0u8, red, alpha);

    let mut green = src.data[1] + mul_div_255_round(src_blend, dst.data[1]);
    green = clamp(0u8, green, alpha);

    let mut blue = src.data[2] + mul_div_255_round(src_blend, dst.data[2]);
    blue = clamp(0u8, blue, alpha);

    Rgba::from_channels(red, green, blue, alpha)
}
