#[macro_use]
extern crate canvas;
extern crate image;

use canvas::canvas::Canvas;
use canvas::types::{Color, RectI, RectF, GPixel};
use canvas::util::{color_to_pixel};
use image::{Rgba, RgbaImage, Pixel};

#[cfg(test)]

fn compare(a: &RgbaImage, b: &RgbaImage, tolerane: i32, verbose: bool) -> f64 {
    // Assert a and b have same dimensions
    assert!(a.dimensions() == b.dimensions()); 

    0.0   
} 

#[test]
fn clear() {
    // Create a new bitmap
    // let bitmap = 

    // Set a pixel in bitmap
    
    // Make a canvas
    // let canvas = Canvas::new(&bitmap);

    // Clear using canvas
}

fn offset(rect: &mut RectF, dx: f32, dy: f32) {
    rect.left += dx;
    rect.right += dx;
    rect.top += dy;
    rect.bottom += dy;
}

fn draw_blend_ramp(canvas: &mut Canvas, bg: &Color) {
    canvas.fill_style = color_to_pixel(bg);
    canvas.clear();

    let mut rect = RectF::xywh(-25.0, -25.0, 70.0, 70.0);

    let delta = 8f32;

    let mut i = 0.0f32;

    while i < 200f32 {
        let r = i as f32 / 200.0;
        let g = (i as f32 / 40.0).cos().abs();
        let b = (i as f32 / 50.0).sin().abs();

        let color = Rgba::from_channels(r, g, b, 0.3);

        canvas.fill_style = GPixel::from(color);
        canvas.fill_rect(&rect, &color);

        offset(&mut rect, delta, delta);

        i += delta;
    }
}

#[test]
fn draw_blend_white() {
    // Create bitmap
    let mut bitmap = RgbaImage::new(200, 200);
    // Create canvas
    let mut canvas = Canvas::new(&mut bitmap);

    draw_blend_ramp(&mut canvas, &Rgba::from_channels(1.0, 1.0, 1.0, 1.0));
}

#[test]
fn draw_blend_black() {
    // Create bitmap
    let mut bitmap = RgbaImage::new(200, 200);
    // Create canvas
    let mut canvas = Canvas::new(&mut bitmap);

    draw_blend_ramp(&mut canvas, &Rgba::from_channels(1.0, 0.0, 0.0, 0.0));
}

#[test]
fn draw_spocks_quad() {
    let mut bitmap = RgbaImage::new(300, 300);
    let mut canvas = Canvas::new(&mut bitmap);

    const N: f32 = 300.0;

    let texture = image::open("tests/spock.png").unwrap();
    
    for y in 0..2 {
        for x in 0..2 {
            // canvas.fill_bitmap_rect(&texture, &RectF::xywh(x as f32 * N - N / 2.0, y as f32 * N - N / 2.0, N, N));
        }
    }
}