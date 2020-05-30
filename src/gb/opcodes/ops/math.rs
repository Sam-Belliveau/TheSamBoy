use crate::gb::cpu::CPU;

// HELPER FUNCTIONS

pub mod alu {
    use crate::gb::cpu::CPU;

    pub fn add_i8_to_u16(cpu: &mut CPU, lhs: u16, rhs: i8) -> u16 {
        let rhs_u16 = rhs as i16 as u16;
        cpu.reg.set_n_flag(false);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag((lhs & 0x000f) + (rhs_u16 & 0x000f) > (0x000f));
        cpu.reg.set_c_flag((lhs & 0x00ff) + (rhs_u16 & 0x00ff) > (0x00ff));
        lhs.wrapping_add(rhs_u16)
    }

    pub fn inc(cpu: &mut CPU, val: u8) -> u8 {
        let ret = val.wrapping_add(1);
        cpu.reg.set_z_flag(ret == 0);
        cpu.reg.set_h_flag((ret & 0x0f) == 0x00);
        cpu.reg.set_n_flag(false);
        ret
    }

    pub fn dec(cpu: &mut CPU, val: u8) -> u8 {
        let ret = val.wrapping_sub(1);
        cpu.reg.set_z_flag(ret == 0);
        cpu.reg.set_h_flag((ret & 0x0f) == 0x0f);
        cpu.reg.set_n_flag(true);
        ret
    }
}


// inc and dec
// inc
pub fn inc_b(cpu: &mut CPU) -> usize {
    cpu.reg.b = alu::inc(cpu, cpu.reg.b);
    4
}

pub fn inc_c(cpu: &mut CPU) -> usize {
    cpu.reg.c = alu::inc(cpu, cpu.reg.c);
    4
}

pub fn inc_d(cpu: &mut CPU) -> usize {
    cpu.reg.d = alu::inc(cpu, cpu.reg.d);
    4
}

pub fn inc_e(cpu: &mut CPU) -> usize {
    cpu.reg.e = alu::inc(cpu, cpu.reg.e);
    4
}

pub fn inc_h(cpu: &mut CPU) -> usize {
    cpu.reg.h = alu::inc(cpu, cpu.reg.h);
    4
}

pub fn inc_l(cpu: &mut CPU) -> usize {
    cpu.reg.l = alu::inc(cpu, cpu.reg.l);
    4
}

pub fn inc_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    let res = alu::inc(cpu, byte);
    cpu.bus.write_byte(hl, res);
    12
}

pub fn inc_a(cpu: &mut CPU) -> usize {
    cpu.reg.a = alu::inc(cpu, cpu.reg.a);
    4
}

// dec
pub fn dec_b(cpu: &mut CPU) -> usize {
    cpu.reg.b = alu::dec(cpu, cpu.reg.b);
    4
}

pub fn dec_c(cpu: &mut CPU) -> usize {
    cpu.reg.c = alu::dec(cpu, cpu.reg.c);
    4
}

pub fn dec_d(cpu: &mut CPU) -> usize {
    cpu.reg.d = alu::dec(cpu, cpu.reg.d);
    4
}

pub fn dec_e(cpu: &mut CPU) -> usize {
    cpu.reg.e = alu::dec(cpu, cpu.reg.e);
    4
}

pub fn dec_h(cpu: &mut CPU) -> usize {
    cpu.reg.h = alu::dec(cpu, cpu.reg.h);
    4
}

pub fn dec_l(cpu: &mut CPU) -> usize {
    cpu.reg.l = alu::dec(cpu, cpu.reg.l);
    4
}

pub fn dec_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    let res = alu::dec(cpu, byte);
    cpu.bus.write_byte(hl, res);
    12
}

pub fn dec_a(cpu: &mut CPU) -> usize {
    cpu.reg.a = alu::dec(cpu, cpu.reg.a);
    4
}