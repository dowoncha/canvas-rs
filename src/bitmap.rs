use pixel::Pixel;

pub struct Bitmap {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
    row_bytes: usize
}
