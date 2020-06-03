
#[derive(Clone)]
pub struct IORegisters {

    pub sb: u8, // Serial transfer data (R/W)
    pub sc: u8, // SIO Control (R/W) 

    pub div: u8, // Divider Register (R/W)

    pub tima: u8, // Timer Counter (R/W)

    pub tma: u8, // Timer Modulo (R/W)
    pub tac: u8, // Timer Control (R/W)

    pub iflag: u8, // Interrupt Flag (R/W)

    /*
     * a TON of unimplemented sound registers
     */

    pub scy: u8, // Scroll Y (R/W)
    pub scx: u8, // Scroll X (R/W)
    
    pub ly: u8,  // LCDC Y-Coordinate (R)
    pub lyc: u8, // LY Compare (R/W)

    pub dma: u8, // SMA Start Address (W)
    
    pub bgp: u8, // BG & Window Palette Data (R/W)
    
    pub obp0: u8, // Object Palette 0 Data (R/W)
    pub obp1: u8, // Object Palette 1 Data (R/W)
    
    pub wy: u8, // Window Y Position (R/W)
    pub wx: u8, // Window X Position (R/W)
    
    pub ienable: u8, // Interrupt Enable (R/W) 
}

impl IORegisters {

    pub fn init() -> Self {
        Self {
            sb: 0,
            sc: 0,

            div: 0,

            tima: 0,

            tma: 0,
            tac: 0,

            iflag: 0,

            /*
            * a TON of unimplemented sound registers
            */

            scy: 0,
            scx: 0,

            ly: 0,
            lyc: 0,

            dma: 0,

            bgp: 0,

            obp0: 0,
            obp1: 0,

            wy: 0,
            wx: 0,

            ienable: 0,
        }
    }
}

impl IORegisters {

    pub fn read_byte(&self, idx: u16) -> u8 {
        
        match idx {
            0xff01 => self.sb,
            0xff02 => self.sc,
            
            _ => {
                println!("Unhandled Read from IO Address [{:#04x?}]", idx);
                0
            }
        }

    }

    pub fn write_byte(&mut self, idx: u16, val: u8) {
        
        match idx {
            0xff01 => self.sb = val,
            0xff02 => self.sc = val,
            
            _ => {
                println!("Unhandled Write to IO Address [{:#04x?}] [val: {:#02x?}]", idx, val);
            },
        }

    }

}