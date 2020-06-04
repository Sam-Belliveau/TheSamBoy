
#[derive(Clone)]
pub struct Timer {

    interrupt: u8,

    div: u8, 
    tima: u8, 
    tma: u8, 
    tac: u8, 

}

impl Timer {

    pub fn init() -> Self {
        Self {
            interrupt: 0,

            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
        }
    }

}

impl Timer {

    pub fn step(&mut self, cycles: usize) {
        // TODO: idk
    }


    pub fn get_interrupt(&mut self) -> u8 {
        let ret = self.interrupt;
        self.interrupt = 0;
        ret
    }
}

impl Timer {

    pub fn read_io_byte(&self, idx: u16) -> u8 {
        match idx {
            0xff04 => self.div,
            0xff05 => self.tima,
            0xff06 => self.tma,
            0xff07 => self.tac,
            

            _ => {
                //println!("Unhandled Timer Read from Address [{:#04x?}]", idx);
                0
            }
        }
    }

    pub fn write_io_byte(&mut self, idx: u16, val: u8) {
        match idx {
            0xff04 => self.div = 0x00,
            0xff05 => self.tima = val,
            0xff06 => self.tma = val,
            0xff07 => self.tac = val,

            _ => {
                println!("Unhandled Timer Read from Address [{:#04x?}] [{:#02x?}]", idx, val);
            }
        }
    }

}