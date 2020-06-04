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
        let mut i = Self {
            rom: Cartridge::load(cartridge),

            gpu: GPU::init(),
            serial: Serial::init(),
            sound: Sound::init(),
            timer: Timer::init(),

            intf: 0,
            inte: 0,

            ram: WorkRAM::init(),
            hram: [0; HIGH_RAM_SIZE],
        };
        
        i.write_byte(0xFF05, 0);
        i.write_byte(0xFF06, 0);
        i.write_byte(0xFF07, 0);
        i.write_byte(0xFF10, 0x80);
        i.write_byte(0xFF11, 0xBF);
        i.write_byte(0xFF12, 0xF3);
        i.write_byte(0xFF14, 0xBF);
        i.write_byte(0xFF16, 0x3F);
        i.write_byte(0xFF16, 0x3F);
        i.write_byte(0xFF17, 0);
        i.write_byte(0xFF19, 0xBF);
        i.write_byte(0xFF1A, 0x7F);
        i.write_byte(0xFF1B, 0xFF);
        i.write_byte(0xFF1C, 0x9F);
        i.write_byte(0xFF1E, 0xFF);
        i.write_byte(0xFF20, 0xFF);
        i.write_byte(0xFF21, 0);
        i.write_byte(0xFF22, 0);
        i.write_byte(0xFF23, 0xBF);
        i.write_byte(0xFF24, 0x77);
        i.write_byte(0xFF25, 0xF3);
        i.write_byte(0xFF26, 0xF1);
        i.write_byte(0xFF40, 0x91);
        i.write_byte(0xFF42, 0);
        i.write_byte(0xFF43, 0);
        i.write_byte(0xFF45, 0);
        i.write_byte(0xFF47, 0xFC);
        i.write_byte(0xFF48, 0xFF);
        i.write_byte(0xFF49, 0xFF);
        i.write_byte(0xFF4A, 0);
        i.write_byte(0xFF4B, 0);
        
        i
    }

}

impl MemoryBus {

    pub fn step(&mut self, cycles: usize) {
        self.gpu.step(cycles);
        self.intf |= self.gpu.get_interrupt();

        self.serial.step(cycles);
        self.intf |= self.serial.get_interrupt();

        self.sound.step(cycles);

        self.timer.step(cycles);
        self.intf |= self.timer.get_interrupt();
    }

    pub fn get_interrupts(&mut self) -> u8 {
        let interrupts: u8 = self.intf & self.inte;

        for i in 0..8 {
            let mask: u8 = 1 << i;

            if (interrupts & mask) != 0 {
                self.intf &= 0xff ^ mask;
                return mask;
            }
        }

        0
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
            0xff46 => 0,
            0xff40..=0xff4f => self.gpu.read_io_byte(idx),

            // High RAM
            0xff80..=0xfffe => self.hram[(idx - 0xff80) as usize],
            
            // Interrupt Enable Register
            0xffff => self.inte,

            _ => {
                // println!("Unhandled Read from Address [{:#04x?}]", idx);
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
            0xff46 => {
                // TODO: Fix This
                // DMA Transfer, 
                let base_addr : u16 = (val as u16) << 8;
                let dest_addr : u16 = 0xfe00;

                for i in 0x00..=0x9f {
                    let source = self.read_byte(base_addr + i);
                    self.write_byte(dest_addr + i, source);
                }
            },
            0xff40..=0xff4f => self.gpu.write_io_byte(idx, val),
            
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
        let h = self.read_byte(idx.wrapping_add(1));
        let l = self.read_byte(idx.wrapping_add(0));

        ((h as u16) << 8) | (l as u16)
    }

    pub fn write_word(&mut self, idx: u16, val: u16) {
        let h = ((val >> 8) & 0xff) as u8;
        let l = ((val >> 0) & 0xff) as u8;

        self.write_byte(idx.wrapping_add(1), h);
        self.write_byte(idx.wrapping_add(0), l);
    }
    
}
