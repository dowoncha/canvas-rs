extern crate canvas;

use canvas::image;
use canvas::image::Pixel;
use canvas::canvas::Canvas;
use canvas::types::{GPixel};
use canvas::canvas::FillStyle;

use std::fs::File;
use std::path::Path;

fn main() {
    let mut bmp = image::RgbaImage::new(150u32, 150u32);

    {
        let mut ctx = Canvas::new(&mut bmp);

        for y in 0..6 {
            for x in 0..6 {
                let red = 255 - (42.5 * y as f32).floor() as u8;
                let green = 255 - (42.5 * x as f32).floor() as u8;

                let style = FillStyle::Rgba(GPixel::from_channels(red, green, 0, 255));

                ctx.set_fill_style(style);
                ctx.fill_rect(x * 25, y * 25, 25, 25 );
            }
        }
    }

    let ref mut fout = File::create(&Path::new("grid.png")).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    let _ = image::ImageRgba8(bmp).save(fout, image::PNG);
}