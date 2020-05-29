use crate::gb::hardware::memory_bus::MemoryBus;
use crate::gb::hardware::registers::Registers;
use crate::gb::opcodes::opfuncs::op;
use crate::gb::opcodes::optable::*;

use std::fs::File;

#[derive(Clone)]
pub struct CPU {
    pub bus: MemoryBus,
    pub reg: Registers,
    pub cycles: usize, 
}

impl CPU {
    pub fn init(cartridge: &mut File) -> Self {
        Self {
            bus: MemoryBus::init(cartridge),
            reg: Registers::init(),
            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        let op = &OP_TABLE[self.read_prog_byte(0) as usize];
        self.reg.pc = self.reg.pc.wrapping_add(op.size);

        let cycles = op.exec(self);

        if cycles == op::UNKNOWN_RETURN_CODE {
            self.reg.pc = self.reg.pc.wrapping_add(1);
            println!("Unimplemented OP Code! {}", op);
        } else {
            self.cycles += cycles;
        }

    }
}

impl CPU {
    
    pub fn stack_push_byte(&mut self, val: u8) {
        self.reg.sp = self.reg.sp.wrapping_sub(1);
        self.bus.write_byte(self.reg.sp, val);
    }

    pub fn stack_push_word(&mut self, val: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.bus.write_word(self.reg.sp, val);
    }

    pub fn stack_pop_byte(&mut self) -> u8 {
        let o = self.bus.read_byte(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(1);
        o
    }

    pub fn stack_pop_word(&mut self) -> u16 {
        let o = self.bus.read_word(self.reg.sp);
        self.reg.sp = self.reg.sp.wrapping_add(2);
        o
    }

}

impl CPU {
    pub fn read_prog_byte(&self, delta: u16) -> u8 {
        self.bus.read_byte(self.reg.pc.wrapping_sub(delta))
    }

    pub fn read_prog_word(&self, delta: u16) -> u16 {
        self.bus.read_word(self.reg.pc.wrapping_sub(delta))
    }
}
