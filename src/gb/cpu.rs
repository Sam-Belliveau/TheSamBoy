use crate::gb::hardware::memory_bus::MemoryBus;
use crate::gb::hardware::registers::Registers;
use crate::gb::opcodes::ops;
use crate::gb::opcodes::table;
use crate::gb::opcodes::opcode::OPCode;

use std::fs::File;

#[derive(Clone)]
pub struct CPU {
    pub bus: MemoryBus,
    pub reg: Registers,

    pub interrupts: bool,
    pub stopped: bool,
    pub halted: bool,

    pub cycles: usize, 
}

impl CPU {
    pub fn init(cartridge: &mut File) -> Self {
        Self {
            bus: MemoryBus::init(cartridge),
            reg: Registers::init(),

            interrupts: true,
            stopped: false,
            halted: false,

            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        if self.halted {
            println!("HALTED");
        } else if self.stopped {
            println!("STOPPED");
        } else {
            let byte = self.read_prog_byte(0);
            let op = &table::OP_TABLE[byte as usize];
    
            self.exec(op);
        }

        self.bus.step(self.cycles);
        self.cycles = 0;
    }

    pub fn exec(&mut self, op: &OPCode) {
        if op.code != self.read_prog_byte(0) {
            panic!("Mismatched OP Code [{}]!", op)
        }

        self.reg.pc = self.reg.pc.wrapping_add(op.size);

        let cycles = op.exec(self);

        if cycles == ops::errors::UNKNOWN_RETURN_CODE {
            print!("EUI OP Code! {}", op);
            //panic!("EUI OP Code! {}", op);
        } else {
            self.cycles += cycles;
            print!("Ran OP Code! {}", op);
        }

        println!("\t\t{}", self.reg);
    }
}

impl CPU {
    pub fn get_rom_name(&self) -> String {
        self.bus.rom.get_name()
    }
}

impl CPU {

    pub fn stack_push(&mut self, val: u16) {
        self.reg.sp = self.reg.sp.wrapping_sub(2);
        self.bus.write_word(self.reg.sp.wrapping_add(1), val);
    }

    pub fn stack_pop(&mut self) -> u16 {
        self.reg.sp = self.reg.sp.wrapping_add(2);
        self.bus.read_word(self.reg.sp.wrapping_sub(1))
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
