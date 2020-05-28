use crate::gb::memory_bus::MemoryBus;
use crate::gb::registers::Registers;

use std::fs::File;

#[derive(Clone)]
pub struct CPU {
    pub bus: MemoryBus,
    pub reg: Registers,
}

impl CPU {
    pub fn init(cartridge: &mut File) -> Self {
        Self {
            bus: MemoryBus::init(cartridge),
            reg: Registers::init(),
        }
    }

    pub fn step(&mut self) {
        let op = self.get_prog_byte();
        self.exec(op);
    }
}

impl CPU {
    pub fn get_prog_byte(&mut self) -> u8 {
        let b = self.bus.read_byte(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(1);
        b
    }

    pub fn get_prog_word(&mut self) -> u16 {
        let w = self.bus.read_word(self.reg.pc);
        self.reg.pc = self.reg.pc.wrapping_add(2);
        w
    }
}

// LINK TO OP TABLE:
// https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
impl CPU {
    pub fn exec(&mut self, op: u8) {
        match op {
            ///// 0x /////

            // NOP
            0x00 => {}

            // LD BC, d16
            0x01 => {
                let word = self.get_prog_word();
                self.reg.set_bc(word);
            }

            // LD (BC), A
            0x02 => {
                let val = self.reg.a;
                let idx = self.reg.get_bc();
                self.bus.write_byte(idx, val);
            }

            ///// 1x /////

            ///// 2x /////

            ///// 3x /////

            ///// 4x /////

            // LD B, B
            0x40 => {
                self.reg.b = self.reg.b;
            }

            // LD B, C
            0x41 => {
                self.reg.b = self.reg.c;
            }

            // LD B, D
            0x42 => {
                self.reg.b = self.reg.d;
            }

            // LD B, E
            0x43 => {
                self.reg.b = self.reg.e;
            }

            // LD B, H
            0x44 => {
                self.reg.b = self.reg.h;
            }

            // LD B, L
            0x45 => {
                self.reg.b = self.reg.l;
            }

            // LD B, (HL)
            0x46 => {
                let idx = self.reg.get_hl();
                self.reg.b = self.bus.read_byte(idx);
            }

            // LD B, A
            0x47 => {
                self.reg.b = self.reg.a;
            }

            // LD C, B
            0x48 => {
                self.reg.c = self.reg.b;
            }

            // LD C, C
            0x49 => {
                self.reg.c = self.reg.c;
            }

            // LD C, D
            0x4a => {
                self.reg.c = self.reg.d;
            }

            // LD C, E
            0x4b => {
                self.reg.c = self.reg.e;
            }

            // LD C, H
            0x4c => {
                self.reg.c = self.reg.h;
            }

            // LD C, L
            0x4d => {
                self.reg.c = self.reg.l;
            }

            // LD C, (HL)
            0x4e => {
                let idx = self.reg.get_hl();
                self.reg.c = self.bus.read_byte(idx);
            }

            // LD C, A
            0x4f => {
                self.reg.c = self.reg.a;
            }

            ///// 5x /////

            // LD D, B
            0x50 => {
                self.reg.d = self.reg.b;
            }

            // LD D, C
            0x51 => {
                self.reg.d = self.reg.c;
            }

            // LD D, D
            0x52 => {
                self.reg.d = self.reg.d;
            }

            // LD D, E
            0x53 => {
                self.reg.d = self.reg.e;
            }

            // LD D, H
            0x54 => {
                self.reg.d = self.reg.h;
            }

            // LD D, L
            0x55 => {
                self.reg.d = self.reg.l;
            }

            // LD D, (HL)
            0x56 => {
                let idx = self.reg.get_hl();
                self.reg.d = self.bus.read_byte(idx);
            }

            // LD D, A
            0x57 => {
                self.reg.d = self.reg.a;
            }

            // LD E, B
            0x58 => {
                self.reg.e = self.reg.b;
            }

            // LD E, C
            0x59 => {
                self.reg.e = self.reg.c;
            }

            // LD E, D
            0x5a => {
                self.reg.e = self.reg.d;
            }

            // LD E, E
            0x5b => {
                self.reg.e = self.reg.e;
            }

            // LD E, H
            0x5c => {
                self.reg.e = self.reg.h;
            }

            // LD E, L
            0x5d => {
                self.reg.e = self.reg.l;
            }

            // LD E, (HL)
            0x5e => {
                let idx = self.reg.get_hl();
                self.reg.e = self.bus.read_byte(idx);
            }

            // LD E, A
            0x5f => {
                self.reg.e = self.reg.a;
            }

            ///// 6x /////

            // LD H, B
            0x60 => {
                self.reg.h = self.reg.b;
            }

            // LD H, C
            0x61 => {
                self.reg.h = self.reg.c;
            }

            // LD H, D
            0x62 => {
                self.reg.h = self.reg.d;
            }

            // LD H, E
            0x63 => {
                self.reg.h = self.reg.e;
            }

            // LD H, H
            0x64 => {
                self.reg.h = self.reg.h;
            }

            // LD H, L
            0x65 => {
                self.reg.h = self.reg.l;
            }

            // LD H, (HL)
            0x66 => {
                let idx = self.reg.get_hl();
                self.reg.h = self.bus.read_byte(idx);
            }

            // LD H, A
            0x67 => {
                self.reg.h = self.reg.a;
            }

            // LD L, B
            0x68 => {
                self.reg.l = self.reg.b;
            }

            // LD L, C
            0x69 => {
                self.reg.l = self.reg.c;
            }

            // LD L, D
            0x6a => {
                self.reg.l = self.reg.d;
            }

            // LD L, E
            0x6b => {
                self.reg.l = self.reg.e;
            }

            // LD L, H
            0x6c => {
                self.reg.l = self.reg.h;
            }

            // LD L, L
            0x6d => {
                self.reg.l = self.reg.l;
            }

            // LD L, (HL)
            0x6e => {
                let idx = self.reg.get_hl();
                self.reg.l = self.bus.read_byte(idx);
            }

            // LD L, A
            0x6f => {
                self.reg.l = self.reg.a;
            }

            ///// 7x /////

            // LD (HL), B
            0x70 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.b);
            }

            // LD (HL), C
            0x71 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.c);
            }

            // LD (HL), D
            0x72 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.d);
            }

            // LD (HL), E
            0x73 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.e);
            }

            // LD (HL), H
            0x74 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.h);
            }

            // LD (HL), L
            0x75 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.l);
            }

            // HALT
            0x76 => {
                println!("Shits Haltin");
                self.reg.pc = self.reg.pc.wrapping_sub(1);
            }

            // LD (HL), A
            0x77 => {
                let idx = self.reg.get_hl();
                self.bus.write_byte(idx, self.reg.b);
            }

            // LD A, B
            0x78 => {
                self.reg.a = self.reg.b;
            }

            // LD A, C
            0x79 => {
                self.reg.a = self.reg.c;
            }

            // LD A, D
            0x7a => {
                self.reg.a = self.reg.d;
            }

            // LD A, E
            0x7b => {
                self.reg.a = self.reg.e;
            }

            // LD A, H
            0x7c => {
                self.reg.a = self.reg.h;
            }

            // LD A, L
            0x7d => {
                self.reg.a = self.reg.l;
            }

            // LD A, (HL)
            0x7e => {
                let idx = self.reg.get_hl();
                self.reg.a = self.bus.read_byte(idx);
            }

            // LD A, A
            0x7f => {
                self.reg.a = self.reg.a;
            }

            ///// 8x /////

            ///// 9x /////

            ///// Ax /////

            ///// Bx /////

            ///// Cx /////

            // JP a16
            0xc3 => {
                self.reg.pc = self.get_prog_word();
            }

            // PREFIX CB
            0xCB => {
                let cb_op = self.get_prog_byte();
                self.exec_cb(cb_op);
            }

            ///// Dx /////

            ///// Ex /////

            ///// Fx /////

            //////////////

            // LD B, C
            0x41 => {
                self.reg.b = self.reg.c;
            }
            // UNHANDLED CODE
            _ => {
                println!("Unhandled OP Code [{:#x?}]", op);
            }
        }
    }

    pub fn exec_cb(&mut self, op: u8) {
        match op {
            _ => {
                println!("Unhandled CB Prefix OP Code [{:#x?}]", op);
            }
        }
    }
}
