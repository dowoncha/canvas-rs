#[macro_use]
extern crate canvas;

use canvas::canvas::{Canvas, FillStyle};
use canvas::image;
use image::{Pixel, RgbaImage};
use canvas::types::GPixel;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut bmp = RgbaImage::new(400u32, 400u32);

    {
        let mut ctx = Canvas::new(&mut bmp);

        let mut style = FillStyle::Rgba(GPixel::from_channels(30, 200, 30, 255));
        ctx.set_fill_style(style);
        ctx.fill_rect(0, 0, 400, 400);

        style = FillStyle::Rgba(GPixel::from_channels(200, 30, 30, 255));
        ctx.set_fill_style(style);
        ctx.fill_rect(100, 100, 100, 100);
    }

    let ref mut fout = File::create(&Path::new("rectangles.png")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = image::ImageRgba8(bmp).save(fout, image::PNG);
}
