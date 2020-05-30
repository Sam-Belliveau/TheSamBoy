
pub mod gb;
pub use crate::gb::cpu::CPU;
use std::{thread, time};
use std::fs::File;

fn main() {
    let mut file = File::open("./tetris.gb").expect("can't open file");
    let mut cpu = CPU::init(&mut file);

    loop {
        let ten_millis = time::Duration::from_millis(0);
        thread::sleep(ten_millis);
        cpu.step();

    }
}
