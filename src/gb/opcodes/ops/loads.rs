use crate::gb::cpu::CPU;
use crate::gb::opcodes::ops::math::alu;

///// 16 bit loads and stores /////

// loading into regs
pub fn ld_bc_d16(cpu: &mut CPU) -> usize {
    cpu.reg.set_bc(cpu.read_prog_word(2));
    12
}

pub fn ld_de_d16(cpu: &mut CPU) -> usize {
    cpu.reg.set_de(cpu.read_prog_word(2));
    12
}

pub fn ld_hl_d16(cpu: &mut CPU) -> usize {
    cpu.reg.set_hl(cpu.read_prog_word(2));
    12
}

pub fn ld_sp_d16(cpu: &mut CPU) -> usize {
    cpu.reg.sp = cpu.read_prog_word(2);
    12
}

// stack stuff
pub fn ld_a16_sp(cpu: &mut CPU) -> usize {
    let addr = cpu.read_prog_word(2);
    cpu.bus.write_word(addr, cpu.reg.sp);
    20
}

pub fn ld_hl_sp_plus_r8(cpu: &mut CPU) -> usize {
    let r8 = cpu.read_prog_byte(1) as i8;
    let sum = alu::add_i8_to_u16(cpu, cpu.reg.sp, r8);
    cpu.reg.set_hl(sum);
    12
}

pub fn ld_sp_hl(cpu: &mut CPU) -> usize {
    cpu.reg.sp = cpu.reg.get_hl();
    8
}


pub fn pop_bc(cpu: &mut CPU) -> usize {
    let res = cpu.stack_pop();
    cpu.reg.set_bc(res);
    12
}

pub fn pop_de(cpu: &mut CPU) -> usize {
    let res = cpu.stack_pop();
    cpu.reg.set_de(res);
    12
}

pub fn pop_hl(cpu: &mut CPU) -> usize {
    let res = cpu.stack_pop();
    cpu.reg.set_hl(res);
    12
}


pub fn pop_af(cpu: &mut CPU) -> usize {
    let res = cpu.stack_pop();
    cpu.reg.set_af(res);
    12
}


pub fn push_bc(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.get_bc());
    16
}

pub fn push_de(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.get_de());
    16
}

pub fn push_hl(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.get_hl());
    16
}

pub fn push_af(cpu: &mut CPU) -> usize {
    cpu.stack_push(cpu.reg.get_af());
    16
}


///// 8 bit loads and stores /////

pub fn ld_bc_a(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_bc();
    cpu.bus.write_byte(addr, cpu.reg.a);
    8
}

pub fn ld_de_a(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_de();
    cpu.bus.write_byte(addr, cpu.reg.a);
    8
}

pub fn ld_hl_inc_a(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_hl();
    cpu.bus.write_byte(addr, cpu.reg.a);
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_add(1));
    8
}

pub fn ld_hl_dec_a(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_hl();
    cpu.bus.write_byte(addr, cpu.reg.a);
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_sub(1));
    8
}


pub fn ld_a_bc(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_bc();
    cpu.reg.a = cpu.bus.read_byte(addr);
    8
}

pub fn ld_a_de(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_de();
    cpu.reg.a = cpu.bus.read_byte(addr);
    8
}

pub fn ld_a_hl_inc(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_hl();
    cpu.reg.a = cpu.bus.read_byte(addr);
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_add(1));
    8
}

pub fn ld_a_hl_dec(cpu: &mut CPU) -> usize {
    let addr = cpu.reg.get_hl();
    cpu.reg.a = cpu.bus.read_byte(addr);
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_sub(1));
    8
}


// Program Loads
pub fn ld_b_d8(cpu: &mut CPU) -> usize {
    cpu.reg.b = cpu.read_prog_byte(1);
    8
}

pub fn ld_c_d8(cpu: &mut CPU) -> usize {
    cpu.reg.c = cpu.read_prog_byte(1);
    8
}

pub fn ld_d_d8(cpu: &mut CPU) -> usize {
    cpu.reg.d = cpu.read_prog_byte(1);
    8
}

pub fn ld_e_d8(cpu: &mut CPU) -> usize {
    cpu.reg.e = cpu.read_prog_byte(1);
    8
}

pub fn ld_h_d8(cpu: &mut CPU) -> usize {
    cpu.reg.h = cpu.read_prog_byte(1);
    8
}

pub fn ld_l_d8(cpu: &mut CPU) -> usize {
    cpu.reg.l = cpu.read_prog_byte(1);
    8
}

pub fn ld_hl_d8(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.read_prog_byte(1);
    cpu.bus.write_byte(hl, byte);
    8
}

pub fn ld_a_d8(cpu: &mut CPU) -> usize {
    cpu.reg.a = cpu.read_prog_byte(1);
    8
}


// Address Loads
pub fn ldh_a8_a(cpu: &mut CPU) -> usize {
    let addr: u16 = 0xff00 + cpu.read_prog_byte(1) as u16;
    cpu.bus.write_byte(addr, cpu.reg.a);
    12
}

pub fn ldh_a_a8(cpu: &mut CPU) -> usize {
    let addr: u16 = 0xff00 + cpu.read_prog_byte(1) as u16;
    cpu.reg.a = cpu.bus.read_byte(addr);
    12
}

pub fn ld_addr_c_a(cpu: &mut CPU) -> usize {
    let addr: u16 = 0xff00 + cpu.reg.c as u16;
    cpu.bus.write_byte(addr, cpu.reg.a);
    12
}

pub fn ld_a_addr_c(cpu: &mut CPU) -> usize {
    let addr: u16 = 0xff00 + cpu.reg.c as u16;
    cpu.reg.a = cpu.bus.read_byte(addr);
    12
}

pub fn ld_a16_a(cpu: &mut CPU) -> usize {
    let addr: u16 = cpu.read_prog_word(2);
    cpu.bus.write_byte(addr, cpu.reg.a);
    12
}

pub fn ld_a_a16(cpu: &mut CPU) -> usize {
    let addr: u16 = cpu.read_prog_word(2);
    cpu.reg.a = cpu.bus.read_byte(addr);
    12
}

// Register Loads (oh boy is there a lot)
pub fn ld_a_b(cpu: &mut CPU) -> usize {
    cpu.reg.a = cpu.reg.b;
    4
}