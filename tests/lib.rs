extern crate canvas_rs;
extern crate image;

use canvas_rs::canvas::Canvas;
use canvas_rs::types::Color;
use image::RgbaImage;

use canvas::canvas::GCanvas;
#[cfg(test)]

#[test]
fn clear() {
    // Create a new bitmap
    let bitmap = 

    // Set a pixel in bitmap
    
    // Make a canvas
    let canvas = Canvas::new(&bitmap);

    // Clear using canvas
}

fn draw_blend_ramp(canvas: &mut Canvas, color: &Color) {

}

#[test]
fn draw_blend_white() {
    // Create bitmap
    let bitmap = RgbaImage::new(200, 200);
    // Create canvas
    let mut canvas = Canvas::new()

    draw_blend_ramp(canvas)
}