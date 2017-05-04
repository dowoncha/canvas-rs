#![crate_type = "lib"]

#![crate_name = "canvas"]

#![allow(non_snake_case)]
#![allow(unused_variables)]

// Might as well reimport image;
pub extern crate image;
extern crate nalgebra;

pub mod canvas;
pub mod edge;
pub mod types;
pub mod rect;
pub mod shader;
pub mod util;

pub mod macros {
    use types::GPixel;

    #[macro_export]
    macro_rules! rgba {
        ($r: expr, $g: expr, $b: expr, $a: expr) => (
            GPixel::from_channels(r, g, b, a)
        )
    }
}

