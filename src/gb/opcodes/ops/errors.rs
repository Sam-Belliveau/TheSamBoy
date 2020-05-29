use crate::gb::cpu::CPU;

pub const UNKNOWN_RETURN_CODE : usize = usize::MAX;

pub fn unknown(_: &mut CPU) -> usize {
    // println!("Error! Unknown OPCode!");
    UNKNOWN_RETURN_CODE
}    

pub fn unused(_: &mut CPU) -> usize {
    println!("WARNING! Unused OPCode was executed!");
    0
}