extern crate minifb;

use minifb::*;

pub mod gb;
pub use crate::gb::cpu::CPU;
use std::{thread, time};
use std::fs::File;

const WIDTH: usize = 256 * 4;
const HEIGHT: usize = 256 * 2;


fn main() {
    let mut file = File::open("./tetris.gb").expect("can't open file");
    let mut cpu = CPU::init(&mut file);

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "tetris",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() {
        cpu.cycles = 0x0000;
        if window.is_key_down(Key::Space) {
            while cpu.cycles < 0x1 {
                cpu.step();
            }
        } else {
            while cpu.cycles < 0x1000 {
                cpu.step();
            }
        }


        for i in 0x0000..=0xffff {
            let byte = cpu.bus.read_byte(i);
            let index = i as usize * 8;

            let mut black : u32 = 0x00000000;
            let mut white : u32 = 0xffffffff;

            if i == cpu.reg.pc {
                black = 0x00ff0000;
                white = 0xff00ff00;
            }

            if i == cpu.reg.sp {
                black = 0x0000ff00;
                white = 0xffff00ff;
            }

            if i == cpu.reg.get_hl() {
                black = 0x000000ff;
                white = 0xffffff00;
            }


            for b in 0..8 {
                let bit_mask = (1 << b) as u8;

                if (byte & bit_mask) != 0 {
                    buffer[(index + b) as usize] = white;
                } else {
                    buffer[(index + b) as usize] = black;
                }
            }
        }
        
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
