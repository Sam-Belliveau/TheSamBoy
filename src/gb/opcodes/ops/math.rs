use crate::gb::cpu::CPU;

// HELPER FUNCTIONS

pub mod alu {
    use crate::gb::cpu::CPU;

    pub mod helper {
        pub fn add_carry(lhs: u8, rhs: u8) -> bool {
            (((lhs & 0xff) as i32) + ((rhs & 0xff) as i32)) > 0xff
        }

        pub fn add_half_carry(lhs: u8, rhs: u8) -> bool {
            (((lhs & 0x0f) as i32) + ((rhs & 0x0f) as i32)) > 0x0f
        }

        pub fn sub_borrow(lhs: u8, rhs: u8) -> bool {
            (((lhs & 0xff) as i32) - ((rhs & 0xff) as i32)) < 0x00
        }

        pub fn sub_half_borrow(lhs: u8, rhs: u8) -> bool {
            (((lhs & 0x0f) as i32) - ((rhs & 0x0f) as i32)) > 0x00
        }
    }

    pub fn add_i8_to_u16(cpu: &mut CPU, lhs: u16, rhs: i8) -> u16 {
        let rhs_u16 = rhs as i16 as u16;
        cpu.reg.set_n_flag(false);
        cpu.reg.set_z_flag(false);
        cpu.reg.set_h_flag((lhs & 0x000f) + (rhs_u16 & 0x000f) > (0x000f));
        cpu.reg.set_c_flag((lhs & 0x00ff) + (rhs_u16 & 0x00ff) > (0x00ff));
        lhs.wrapping_add(rhs_u16)
    }

    // inc / dec
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

    // Adding
    pub fn add_a_n(cpu: &mut CPU, n: u8) {
        let res = cpu.reg.a.wrapping_add(n);

        cpu.reg.set_z_flag(cpu.reg.a == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(helper::add_half_carry(cpu.reg.a, n));
        cpu.reg.set_c_flag(helper::add_carry(cpu.reg.a, n));

        cpu.reg.a = res;
    }

    pub fn adc_a_n(cpu: &mut CPU, n: u8) {
        if cpu.reg.get_c_flag() {
            add_a_n(cpu, n.wrapping_add(1))
        } else {
            add_a_n(cpu, n)
        }
    }

    // subtracting
    pub fn sub_n(cpu: &mut CPU, n: u8) {
        let res = cpu.reg.a.wrapping_add(n);

        cpu.reg.set_z_flag(res == 0);
        cpu.reg.set_n_flag(true);
        cpu.reg.set_h_flag(!helper::sub_half_borrow(cpu.reg.a, n));
        cpu.reg.set_c_flag(!helper::sub_borrow(cpu.reg.a, n));

        cpu.reg.a = res;
    }

    pub fn sbc_a_n(cpu: &mut CPU, n: u8) {
        if cpu.reg.get_c_flag() {
            sub_n(cpu, n.wrapping_add(1))
        } else {
            sub_n(cpu, n)
        }
    }

    // Bitwise OPs
    pub fn and_n(cpu: &mut CPU, n: u8) {
        cpu.reg.a = cpu.reg.a & n;

        cpu.reg.set_z_flag(cpu.reg.a == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(true);
        cpu.reg.set_c_flag(false);
    }

    pub fn xor_n(cpu: &mut CPU, n: u8) {
        cpu.reg.a = cpu.reg.a ^ n;

        cpu.reg.set_z_flag(cpu.reg.a == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_c_flag(false);
    }

    pub fn or_n(cpu: &mut CPU, n: u8) {
        cpu.reg.a = cpu.reg.a | n;

        cpu.reg.set_z_flag(cpu.reg.a == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_c_flag(false);
    }

    pub fn cp_n(cpu: &mut CPU, n: u8) {
        let ret = cpu.reg.a.wrapping_sub(n);

        println!("CP A < {:#02x?}", n);
        cpu.reg.set_z_flag(ret == 0);
        cpu.reg.set_n_flag(true);
        cpu.reg.set_h_flag(!helper::sub_half_borrow(cpu.reg.a, n));
        cpu.reg.set_c_flag(!helper::sub_borrow(cpu.reg.a, n));
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

pub fn inc_addr_hl(cpu: &mut CPU) -> usize {
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

pub fn dec_addr_hl(cpu: &mut CPU) -> usize {
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

// Misc
pub fn daa(cpu: &mut CPU) -> usize {
    let mut adjust : u8 = if cpu.reg.get_c_flag() { 0x60 } else { 0x00 };
    
    if cpu.reg.get_h_flag() {
        adjust |= 0x6;
    }

    if cpu.reg.get_n_flag() {
        cpu.reg.a = cpu.reg.a.wrapping_sub(adjust);
    } else {
        if (cpu.reg.a & 0x0f) > 0x09 { adjust |= 0x06; }
        if (cpu.reg.a & 0xff) > 0x99 { adjust |= 0x60; }
        cpu.reg.a = cpu.reg.a.wrapping_add(adjust);
    }

    cpu.reg.set_z_flag(cpu.reg.a == 0);
    cpu.reg.set_h_flag(false);
    cpu.reg.set_c_flag(adjust >= 0x60);

    4
}

pub fn cpl(cpu: &mut CPU) -> usize {
    cpu.reg.a = 0xff ^ cpu.reg.a;

    cpu.reg.set_n_flag(true);
    cpu.reg.set_h_flag(true);
    
    4
}

pub fn ccf(cpu: &mut CPU) -> usize {
    cpu.reg.set_n_flag(false);
    cpu.reg.set_h_flag(false);
    cpu.reg.set_h_flag(!cpu.reg.get_c_flag());
    
    4
}

pub fn scf(cpu: &mut CPU) -> usize {
    cpu.reg.set_n_flag(false);
    cpu.reg.set_h_flag(false);
    cpu.reg.set_h_flag(true);
    
    4
}

// Adding
pub fn add_a_b(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.b);
    4
}

pub fn add_a_c(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.c);
    4
}

pub fn add_a_d(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.d);
    4
}

pub fn add_a_e(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.e);
    4
}

pub fn add_a_h(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.h);
    4
}

pub fn add_a_l(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.l);
    4
}

pub fn add_a_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::add_a_n(cpu, byte);
    8
} 

pub fn add_a_a(cpu: &mut CPU) -> usize {
    alu::add_a_n(cpu, cpu.reg.a);
    4
}

pub fn add_a_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::add_a_n(cpu, byte);
    8
} 

// Adding With Carry
pub fn adc_a_b(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.b);
    4
}

pub fn adc_a_c(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.c);
    4
}

pub fn adc_a_d(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.d);
    4
}

pub fn adc_a_e(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.e);
    4
}

pub fn adc_a_h(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.h);
    4
}

pub fn adc_a_l(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.l);
    4
}

pub fn adc_a_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::adc_a_n(cpu, byte);
    8
} 

pub fn adc_a_a(cpu: &mut CPU) -> usize {
    alu::adc_a_n(cpu, cpu.reg.a);
    4
}

pub fn adc_a_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::adc_a_n(cpu, byte);
    8
} 

// Subing
pub fn sub_b(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.b);
    4
}

pub fn sub_c(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.c);
    4
}

pub fn sub_d(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.d);
    4
}

pub fn sub_e(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.e);
    4
}

pub fn sub_h(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.h);
    4
}

pub fn sub_l(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.l);
    4
}

pub fn sub_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::sub_n(cpu, byte);
    8
} 

pub fn sub_a(cpu: &mut CPU) -> usize {
    alu::sub_n(cpu, cpu.reg.a);
    4
}

pub fn sub_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::sub_n(cpu, byte);
    8
} 

// Subing With Carry
pub fn sbc_a_b(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.b);
    4
}

pub fn sbc_a_c(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.c);
    4
}

pub fn sbc_a_d(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.d);
    4
}

pub fn sbc_a_e(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.e);
    4
}

pub fn sbc_a_h(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.h);
    4
}

pub fn sbc_a_l(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.l);
    4
}

pub fn sbc_a_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::sbc_a_n(cpu, byte);
    8
} 

pub fn sbc_a_a(cpu: &mut CPU) -> usize {
    alu::sbc_a_n(cpu, cpu.reg.a);
    4
}

pub fn sbc_a_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::sbc_a_n(cpu, byte);
    8
} 

// And
pub fn and_b(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.b);
    4
}

pub fn and_c(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.c);
    4
}

pub fn and_d(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.d);
    4
}

pub fn and_e(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.e);
    4
}

pub fn and_h(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.h);
    4
}

pub fn and_l(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.l);
    4
}

pub fn and_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::and_n(cpu, byte);
    8
} 

pub fn and_a(cpu: &mut CPU) -> usize {
    alu::and_n(cpu, cpu.reg.a);
    4
}

pub fn and_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::and_n(cpu, byte);
    8
} 

// Xor
pub fn xor_b(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.b);
    4
}

pub fn xor_c(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.c);
    4
}

pub fn xor_d(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.d);
    4
}

pub fn xor_e(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.e);
    4
}

pub fn xor_h(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.h);
    4
}

pub fn xor_l(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.l);
    4
}

pub fn xor_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::xor_n(cpu, byte);
    8
} 

pub fn xor_a(cpu: &mut CPU) -> usize {
    alu::xor_n(cpu, cpu.reg.a);
    4
}

pub fn xor_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::xor_n(cpu, byte);
    8
} 

// Or
pub fn or_b(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.b);
    4
}

pub fn or_c(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.c);
    4
}

pub fn or_d(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.d);
    4
}

pub fn or_e(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.e);
    4
}

pub fn or_h(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.h);
    4
}

pub fn or_l(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.l);
    4
}

pub fn or_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::or_n(cpu, byte);
    8
} 

pub fn or_a(cpu: &mut CPU) -> usize {
    alu::or_n(cpu, cpu.reg.a);
    4
}

pub fn or_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::or_n(cpu, byte);
    8
} 

// CP
pub fn cp_b(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.b);
    4
}

pub fn cp_c(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.c);
    4
}

pub fn cp_d(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.d);
    4
}

pub fn cp_e(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.e);
    4
}

pub fn cp_h(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.h);
    4
}

pub fn cp_l(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.l);
    4
}

pub fn cp_hl(cpu: &mut CPU) -> usize {
    let hl = cpu.reg.get_hl();
    let byte = cpu.bus.read_byte(hl);
    alu::cp_n(cpu, byte);
    8
} 

pub fn cp_a(cpu: &mut CPU) -> usize {
    alu::cp_n(cpu, cpu.reg.a);
    4
}

pub fn cp_d8(cpu: &mut CPU) -> usize {
    let byte = cpu.read_prog_byte(1);
    alu::cp_n(cpu, byte);
    8
} 

// 16 bit math
pub fn inc_bc(cpu: &mut CPU) -> usize {
    cpu.reg.set_bc(cpu.reg.get_bc().wrapping_add(1));
    8
} 

pub fn inc_de(cpu: &mut CPU) -> usize {
    cpu.reg.set_de(cpu.reg.get_de().wrapping_add(1));
    8
} 

pub fn inc_hl(cpu: &mut CPU) -> usize {
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_add(1));
    8
} 

pub fn inc_sp(cpu: &mut CPU) -> usize {
    cpu.reg.sp = (cpu.reg.sp.wrapping_add(1));
    8
} 


pub fn dec_bc(cpu: &mut CPU) -> usize {
    cpu.reg.set_bc(cpu.reg.get_bc().wrapping_sub(1));
    8
} 

pub fn dec_de(cpu: &mut CPU) -> usize {
    cpu.reg.set_de(cpu.reg.get_de().wrapping_sub(1));
    8
} 

pub fn dec_hl(cpu: &mut CPU) -> usize {
    cpu.reg.set_hl(cpu.reg.get_hl().wrapping_sub(1));
    8
} 

pub fn dec_sp(cpu: &mut CPU) -> usize {
    cpu.reg.sp = (cpu.reg.sp.wrapping_sub(1));
    8
} 