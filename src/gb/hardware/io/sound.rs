
#[derive(Clone)]
pub struct Sound {

}

impl Sound {

    pub fn init() -> Self {
        Self {
        }
    }

}

impl Sound {

    pub fn step(&mut self, cycles: usize) {
        // TODO: idk
    }

}

impl Sound {

    pub fn read_io_byte(&self, idx: u16) -> u8 {
        match idx {
            
            _ => {
                //println!("Unhandled Sound Read from Address [{:#04x?}]", idx);
                0
            }
        }
    }

    pub fn write_io_byte(&mut self, idx: u16, val: u8) {
        match idx {
            
            _ => {
                println!("Unhandled Sound Write from Address [{:#04x?}] [{:#02x?}]", idx, val);
            }
        }
    }

}