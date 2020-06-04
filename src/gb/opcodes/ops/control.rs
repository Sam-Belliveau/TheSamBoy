use crate::gb::cpu::CPU;
use crate::gb::opcodes::table;

pub fn nop(_: &mut CPU) -> usize {
    4
}

pub fn stop(cpu: &mut CPU) -> usize {
    // halt CPU & LCD display until button pressed.
    if cpu.interrupts {
        cpu.stopped = true;
    }
    4
}

pub fn halt(cpu: &mut CPU) -> usize {
    // Power down CPU until an interrupt occurs. 
    // Use this  when ever possible to reduce energy consumption
    if cpu.interrupts {
        cpu.halted = true;
    }
    4
}

pub fn prefix_cb(cpu: &mut CPU) -> usize  {
    let code = cpu.read_prog_byte(0);
    let op = &table::CB_OP_TABLE[code as usize];
    
    cpu.exec(op);

    4
}

pub fn di(cpu: &mut CPU) -> usize {
    cpu.interrupts = false;
    4
}

pub fn ei(cpu: &mut CPU) -> usize {
    cpu.interrupts = true;
    4
}