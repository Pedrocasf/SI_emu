extern crate i8080_core;
pub mod si;

use i8080_core::cpu::CPU;
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
    window.set_target_fps(60);
    let mut cpu = CPU::new(None,None);
    let mut s_i = SI::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut i = 0;
        while i < 8333 {
            i += s_i.run_instr(&mut cpu);
        }
        for (i,px) in buffer.iter_mut().enumerate() {
            if i == WIDTH * HEIGHT / 2{
                break;
            }
            *px = if s_i.get_px(i as u16) {0xFFFFFFFF} else{0x00000000}
        }
        s_i.interrupt(&mut cpu, 0xCF);
        i = 0;
        while i<8333{
            i += s_i.run_instr(&mut cpu);
        }
        for (i,px) in buffer.iter_mut().enumerate().skip(WIDTH*HEIGHT/2) {
            if i == WIDTH * HEIGHT{
                break;
            }
            *px = if s_i.get_px(i as u16) {0xFFFFFFFF} else{0x00000000}
        }
        //s_i.interrupt(0xD7);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}