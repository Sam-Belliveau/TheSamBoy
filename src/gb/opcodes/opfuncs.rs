pub mod op {
    use crate::gb::cpu::CPU;
    use crate::gb::opcodes::optable::CB_OP_TABLE;
    use std::usize;

    pub const UNKNOWN_RETURN_CODE : usize = usize::MAX;

    pub fn unknown(_: &mut CPU) -> usize {
        // println!("Error! Unknown OPCode!");
        UNKNOWN_RETURN_CODE
    }    

    pub fn unused(_: &mut CPU) -> usize {
        println!("WARNING! Unused OPCode was executed!");
        0
    }
    
    pub fn nop(_: &mut CPU) -> usize {
        4
    }
    
    pub fn prefix_cb(cpu: &mut CPU) -> usize  {
        // Get next OP Code
        let code = cpu.read_prog_byte(0);

        let op = &CB_OP_TABLE[code as usize];
        cpu.reg.pc = cpu.reg.pc.wrapping_add(op.size);
        
        let cycles = op.exec(cpu);

        if cycles == UNKNOWN_RETURN_CODE {
            cpu.reg.pc = cpu.reg.pc.wrapping_add(1);
            println!("Unimplemented CB Code! {}", op);
            4
        } else {
            4 + cycles
        }
    }
}
