use std::fmt;

#[derive(Debug, Clone)]
pub struct Registers {
    pub pc: u16, // Program Counter
    pub sp: u16, // Stack Pointer

    pub a: u8, // Accumulator
    pub f: u8, // Flag Register

    // General Purpose Flags
    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8, // High
    pub l: u8, // Low
}

impl Registers {
    pub fn init() -> Self {
        Self {
            pc: 0x0000,
            sp: 0xfffe,

            a: 0x00, f: 0x00,

            b: 0x00, c: 0x00,

            d: 0x00, e: 0x00,

            h: 0x00, l: 0x00,
        }
    }
}

impl Registers {
    fn get_flag_bit(&self, bit: usize) -> bool {
        (self.f & ((1 << bit) as u8)) != 0
    }

    fn set_flag_bit(&mut self, bit: usize, val: bool) {
        if val {
            self.f |= (1 << bit) as u8;
        } else {
            self.f &= 0xff ^ ((1 << bit) as u8);
        }
    }


    pub fn get_z_flag(&self) -> bool {
        self.get_flag_bit(7)
    }

    pub fn set_z_flag(&mut self, val: bool) {
        self.set_flag_bit(7, val);
    }


    pub fn get_n_flag(&self) -> bool {
        self.get_flag_bit(6)
    }

    pub fn set_n_flag(&mut self, val: bool) {
        self.set_flag_bit(6, val);
    }


    pub fn get_h_flag(&self) -> bool {
        self.get_flag_bit(5)
    }

    pub fn set_h_flag(&mut self, val: bool) {
        self.set_flag_bit(5, val);
    }


    pub fn get_c_flag(&self) -> bool {
        self.get_flag_bit(4)
    }

    pub fn set_c_flag(&mut self, val: bool) {
        self.set_flag_bit(4, val);
    }
}

impl Registers {

    // Getting 16 bit values
    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    // Setting 16 bit values
    pub fn set_af(&mut self, val: u16) {
        self.a = ((val >> 8) & 0xff) as u8;
        self.f = ((val >> 0) & 0xff) as u8;
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val >> 8) & 0xff) as u8;
        self.c = ((val >> 0) & 0xff) as u8;
    }
    
    pub fn set_de(&mut self, val: u16) {
        self.d = ((val >> 8) & 0xff) as u8;
        self.e = ((val >> 0) & 0xff) as u8;
    }
    
    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val >> 8) & 0xff) as u8;
        self.l = ((val >> 0) & 0xff) as u8;
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[pc: {:#06x}, sp: {:#06x}, a: {:#04x}, f: {:#04x}, b: {:#04x}, c: {:#04x}, d: {:#04x}, e: {:#04x}, h: {:#04x}, l: {:#04x}]", 
            self.pc, self.sp, 
            self.a, self.f,
            self.b, self.c,
            self.d, self.e,
            self.h, self.l
        )
    }
}