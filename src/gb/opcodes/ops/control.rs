use crate::gb::cpu::CPU;
use crate::gb::opcodes::table;
use crate::gb::opcodes::ops::errors;

pub fn nop(_: &mut CPU) -> usize {
    4
}

pub fn stop(_: &mut CPU) -> usize {
    // halt CPU & LCD display until button pressed.
    4
}

pub fn halt(_: &mut CPU) -> usize {
    // Power down CPU until an interrupt occurs. 
    // Use this  when ever possible to reduce energy consumption
    4
}

pub fn prefix_cb(cpu: &mut CPU) -> usize  {
    // Get next OP Code
    let code = cpu.read_prog_byte(0);

    let op = &table::CB_OP_TABLE[code as usize];
    cpu.reg.pc = cpu.reg.pc.wrapping_add(op.size);
    
    let cycles = op.exec(cpu);

    if cycles == errors::UNKNOWN_RETURN_CODE {
        cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
        println!("Unimplemented CB Code! {}", op);
        4
    } else {
        4 + cycles
    }
}

pub fn di(_: &mut CPU) -> usize {
    // Disables Interrupts
    4
}

pub fn ei(_: &mut CPU) -> usize {
    // Enable Interrupts
    4
}