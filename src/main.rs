
pub mod gb;
pub use crate::gb::cpu::CPU;

use std::fs::File;

fn main() {
    let mut file = File::open("./tetris.gb").expect("can't open file");
    let mut cpu = CPU::init(&mut file);

    for i in 0..100 {
        cpu.step();
    }
}
