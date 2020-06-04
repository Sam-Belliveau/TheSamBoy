use crate::gb::cpu::CPU;

pub mod blu {
    use crate::gb::cpu::CPU;

    pub fn rlc_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn rrc_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn rl_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn rr_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn sla_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn sra_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn swap_n(cpu: &mut CPU, n: u8) -> u8 {
        let res = ((n & 0x0f).wrapping_shl(4)) | ((n & 0xf0).wrapping_shr(4)) as u8;

        cpu.reg.set_z_flag(res == 0);
        cpu.reg.set_n_flag(false);
        cpu.reg.set_h_flag(false);
        cpu.reg.set_c_flag(false);

        res
    }

    pub fn srl_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn bit_b_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn res_b_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

    pub fn set_b_n(cpu: &mut CPU, n: u8) -> u8 {
        0
    }

}

pub fn swap_a(cpu: &mut CPU) -> usize {
    cpu.reg.a = blu::swap_n(cpu, cpu.reg.a);
    8
}