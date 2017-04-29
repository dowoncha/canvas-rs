use image::Rgba;

pub trait Shader {
    // fn set_context(ctm: Mat) -> bool;

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