use crate::gb::cpu::CPU;

use std::fmt;

type OPFunction = fn(&mut CPU) -> usize;

pub struct OPCode {
    pub code: u8,
    pub name: &'static str,
    pub size: u16,
    pub func: OPFunction,
}

impl OPCode {

    pub fn exec(&self, cpu: &mut CPU) -> usize {
        (self.func)(cpu)
    }

}

impl fmt::Display for OPCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:#04x}] {}", self.code, self.name)
    }
}