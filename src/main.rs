extern crate i8080_core;
pub mod si;
use si::SI;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    #[cfg(feature = "log")]
    simple_logger::init_with_level(log::Level::Error).unwrap();
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16666)));
    let mut s_i = SI::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in 0..4167{
            s_i.run_instr();
        }
        for (i,px) in buffer.iter_mut().enumerate() {
            if i == WIDTH * HEIGHT / 2{
                break;
            }
            *px = if s_i.get_px(i as u16) {0xFFFFFFFF} else{0x00000000}
        }
        s_i.interrupt(0xCF);
        
        for i in 0..4167{
            s_i.run_instr();
        }
        for (i,px) in buffer.iter_mut().enumerate().skip(WIDTH*HEIGHT/2) {
            if i == WIDTH * HEIGHT{
                break;
            }
            *px = if s_i.get_px(i as u16) {0xFFFFFFFF} else{0x00000000}
        }
        s_i.interrupt(0xD7);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}