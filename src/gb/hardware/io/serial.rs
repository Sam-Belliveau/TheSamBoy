
#[derive(Clone)]
pub struct Serial {

    interrupt: u8,

    sb: u8, 
    sc: u8,

}

impl Serial {

    pub fn init() -> Self {
        Self {
            interrupt: 0,

            sb: 0,
            sc: 0,
        }
    }

}

impl Serial {

    pub fn step(&mut self, cycles: usize) {
        // TODO: idk
    }

    pub fn get_interrupt(&mut self) -> u8 {
        let ret = self.interrupt;
        self.interrupt = 0;
        ret
    }

}

impl Serial {

    pub fn read_io_byte(&self, idx: u16) -> u8 {
        match idx {
            0xff01 => self.sb,
            0xff02 => self.sc,
            
            _ => {
                //println!("Unhandled Serial Read from Address [{:#04x?}]", idx);
                0
            }
        }
    }

    pub fn write_io_byte(&mut self, idx: u16, val: u8) {
        match idx {
            0xff01 => self.sb = val,
            0xff02 => self.sc = val,

            _ => {
                println!("Unhandled Serial Read from Address [{:#04x?}] [{:#02x?}]", idx, val);
            }
        }
    }

}