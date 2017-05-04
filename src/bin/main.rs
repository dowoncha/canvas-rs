#[allow(unused_variables)]
#[allow(unused_imports)]
#[allow(unused_mut)]

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("Raytracer".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();

    let (window, mut device, mut factory, main_color, main_depth)
        = gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);

    'main: loop {
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {}
            }
        }

        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
