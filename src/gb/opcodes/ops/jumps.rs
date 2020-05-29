use crate::gb::cpu::CPU;
use super::control;

// Jump Relative (takes in 8bit num)
pub fn jr_r8(cpu: &mut CPU) -> usize {
    let r8 = cpu.read_prog_byte(1) as i8;
    cpu.reg.pc = ((cpu.reg.pc as u32 as i32) + (r8 as i32)) as u16; 
    12
}

pub fn jr_nz_r8(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_z_flag() { jr_r8(cpu) }
    else { 8 }
}

pub fn jr_z_r8(cpu: &mut CPU) -> usize {
    if cpu.reg.get_z_flag() { jr_r8(cpu) }
    else { 8 }
}

pub fn jr_nc_r8(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_c_flag() { jr_r8(cpu) }
    else { 8 }
}

pub fn jr_c_r8(cpu: &mut CPU) -> usize {
    if cpu.reg.get_c_flag() { jr_r8(cpu) }
    else { 8 }
}

// Return from function (pops previous location from stack)
pub fn ret(cpu: &mut CPU) -> usize {
    cpu.reg.pc = cpu.stack_pop(); 20
}

pub fn reti(cpu: &mut CPU) -> usize {
    let o = ret(cpu);
    control::ei(cpu); o
}

pub fn ret_nz(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_z_flag() { ret(cpu) } 
    else { 8 }
}

pub fn ret_z(cpu: &mut CPU) -> usize {
    if cpu.reg.get_z_flag() { ret(cpu) } 
    else { 8 }
}

pub fn ret_nc(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_c_flag() { ret(cpu) } 
    else { 8 }
}

pub fn ret_c(cpu: &mut CPU) -> usize {
    if cpu.reg.get_c_flag() { ret(cpu) } 
    else { 8 }
}

// Call, jumps to address and pushes current address to stack
pub fn call_a16(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = cpu.read_prog_word(2); 
    24
}

pub fn call_nz_a16(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_z_flag() { call_a16(cpu) } 
    else { 12 }
}

pub fn call_z_a16(cpu: &mut CPU) -> usize {
    if cpu.reg.get_z_flag() { call_a16(cpu) } 
    else { 12 }
}

pub fn call_nc_a16(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_c_flag() { call_a16(cpu) } 
    else { 12 }
}

pub fn call_c_a16(cpu: &mut CPU) -> usize {
    if cpu.reg.get_c_flag() { call_a16(cpu) } 
    else { 12 }
}

// Jump to address (takes 16 bit address)
pub fn jp_a16(cpu: &mut CPU) -> usize {
    cpu.reg.pc = cpu.read_prog_word(2); 16
}

pub fn jp_hl(cpu: &mut CPU) -> usize {
    // TODO: Check THis
    cpu.reg.pc = cpu.read_prog_word(cpu.reg.get_hl()); 4
}

pub fn jp_nz_a16(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_z_flag() { jp_a16(cpu) }
    else { 12 }
}

pub fn jp_z_a16(cpu: &mut CPU) -> usize {
    if cpu.reg.get_z_flag() { jp_a16(cpu) }
    else { 12 }
}

pub fn jp_nc_a16(cpu: &mut CPU) -> usize {
    if !cpu.reg.get_c_flag() { jp_a16(cpu) }
    else { 12 }
}

pub fn jp_c_a16(cpu: &mut CPU) -> usize {
    if cpu.reg.get_c_flag() { jp_a16(cpu) }
    else { 12 }
}

// Retarts (push addr to stack and jump to address)
pub fn rst_00h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0000; 32
}

pub fn rst_08h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0008; 32
}

pub fn rst_10h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0010; 32
}

pub fn rst_18h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0018; 32
}

pub fn rst_20h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0020; 32
}

pub fn rst_28h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0028; 32
}

pub fn rst_30h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0030; 32
}

pub fn rst_38h(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.pc);
    cpu.reg.pc = 0x0038; 32
}