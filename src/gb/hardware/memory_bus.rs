use crate::gb::hardware::cartridge::Cartridge;
use crate::gb::hardware::work_ram::WorkRAM;

use crate::gb::hardware::io::gpu::GPU;
use crate::gb::hardware::io::serial::Serial;
use crate::gb::hardware::io::sound::Sound;
use crate::gb::hardware::io::timer::Timer;

use std::fs::File;

////////// HIGH RAM //////////
const HIGH_RAM_SIZE : usize = 128;
pub type HighRAM = [u8; HIGH_RAM_SIZE];

////////// MEMORY BUS //////////
#[derive(Clone)]
pub struct MemoryBus {
    pub rom: Cartridge,
    
    pub gpu: GPU,
    pub serial: Serial,
    pub sound: Sound,
    pub timer: Timer,

    pub intf: u8,
    pub inte: u8,

    pub ram: WorkRAM,
    pub hram: HighRAM,
}

impl MemoryBus {

    // initialize everything with default values and rom
    pub fn init(cartridge: &mut File) -> Self {
        Self {
            rom: Cartridge::load(cartridge),

            gpu: GPU::init(),
            serial: Serial::init(),
            sound: Sound::init(),
            timer: Timer::init(),

            intf: 0,
            inte: 0,

            ram: WorkRAM::init(),
            hram: [0; HIGH_RAM_SIZE],
        }
    }

}

impl MemoryBus {

    pub fn step(&mut self, cycles: usize) {

        self.gpu.step(cycles);
        self.serial.step(cycles);
        self.sound.step(cycles);
        self.timer.step(cycles);

    }

}

impl MemoryBus {
    // read byte from memory map
    pub fn read_byte(&self, idx: u16) -> u8 {
        match idx {
            // 16kb ROM Bank 00
            0x0000..=0x3fff => self.rom.read_byte(idx),

            // 16kb ROM Bank 01..NN
            0x4000..=0x7fff => self.rom.read_bank_byte(idx - 0x4000),

            // 8kb Video RAM
            0x8000..=0x9fff => self.gpu.read_vram_byte(idx - 0x8000),
            
            // 8kb External RAM
            0xa000..=0xbfff => self.rom.read_ram_byte(idx - 0xa000),
            
            // 4kb Work RAM Bank 0
            0xc000..=0xcfff => self.ram.read_byte(idx - 0xc000),
            
            // 4kb Work RAM Bank NN
            0xd000..=0xdfff => self.ram.read_bank_byte(idx - 0xd000),
            
            // ECHO Space (0xc000 - 0xddff)
            0xe000..=0xfdff => self.read_byte(idx - 0x2000),
            
            // Sprite Attribute Table (OAM)
            // 0xfe00..=0xfe9f => 0, // TODO: figure this out
            
            // Not Usable
            // 0xfea0..=0xfeff => 0, // TODO: figure this out
            
            // I/O Ports
            0xff00 => self.gpu.read_io_byte(idx),
            0xff01..=0xff02 => self.serial.read_io_byte(idx),
            0xff01..=0xff0e => self.timer.read_io_byte(idx),
            0xff0f => self.intf,
            0xff10..=0xff3f => self.sound.read_io_byte(idx),
            0xff40..=0xff45 => self.gpu.read_io_byte(idx),

            // High RAM
            0xff80..=0xfffe => self.hram[(idx - 0xff80) as usize],
            
            // Interrupt Enable Register
            0xffff => self.inte,

            _ => {
                println!("Unhandled Read from Address [{:#04x?}]", idx);
                0
            }
        }
    }

    // write byte to memory map
    pub fn write_byte(&mut self, idx: u16, val: u8) {
        match idx {
            // Switch ROM Bank
            0x2000 => self.rom.set_bank(val),

            // 8kb Video RAM
            0x8000..=0x9fff => self.gpu.write_vram_byte(idx - 0x8000, val),
            
            // 8kb External RAM
            0xa000..=0xbfff => self.rom.write_ram_byte(idx - 0xa000, val),
            
            // 4kb Work RAM Bank 0
            0xc000..=0xcfff => self.ram.write_byte(idx - 0xc000, val),
            
            // 4kb Work RAM Bank NN
            0xd000..=0xdfff => self.ram.write_bank_byte(idx - 0xd000, val),
            
            // ECHO Space
            0xe000..=0xfdff => self.write_byte(idx - 0x2000, val),
            
            // Sprite Attribute Table (OAM)
            // 0xfe00..=0xfe9f => 0, // TODO: figure this out
            
            // Not Usable
            // 0xfea0..=0xfeff => 0, // TODO: figure this out
            
            // I/O Ports
            0xff00 => self.gpu.write_io_byte(idx, val),
            0xff01..=0xff02 => self.serial.write_io_byte(idx, val),
            0xff01..=0xff0e => self.timer.write_io_byte(idx, val),
            0xff0f => self.intf = val,
            0xff10..=0xff3f => self.sound.write_io_byte(idx, val),
            0xff40..=0xff45 => self.gpu.write_io_byte(idx, val),
            
            // High RAM
            0xff80..=0xfffe => self.hram[(idx - 0xff80) as usize] = val,
            
            // Interrupt Enable Register
            0xffff => self.inte = val,

            _ => {
                println!("Unhandled Write to Address [{:#04x?}] [val: {:#02x?}]", idx, val);
            },
        }
    }

}

impl MemoryBus {

    pub fn read_word(&self, idx: u16) -> u16 {
        let h = self.read_byte(idx + 1);
        let l = self.read_byte(idx + 0);

        ((h as u16) << 8) | (l as u16)
    }

    pub fn write_word(&mut self, idx: u16, val: u16) {
        let h = ((val >> 8) & 0xff) as u8;
        let l = ((val >> 0) & 0xff) as u8;

        self.write_byte(idx + 1, h);
        self.write_byte(idx + 0, l);
    }
    
}
