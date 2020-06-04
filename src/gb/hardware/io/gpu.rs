
use minifb::{WindowOptions, Window, Key};
use std::rc::Rc;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

const VRAM_BANK_NUM : usize = 2;
const VRAM_BANK_SIZE : usize = 0x2000;

type VRAMBank = [u8; VRAM_BANK_SIZE];

#[derive(Clone)]
pub struct GPU {

    fbuffer: Vec<u32>,
    window: Rc<Window>,

    keypad: u8,
    
    cycle: usize,

    vram_bank: u8,
    vram_banks: Vec<VRAMBank>,

    interrupt: u8,

    // IO Registers
    ldcd: u8,
    stat: u8,
    
    scy: u8,
    scx: u8,

    ly: u8,
    lyc: u8,

    bgp: u8,

    obp0: u8,
    obp1: u8,

    wy: u8,
    wx: u8,
}

impl GPU {

    // Make new GPU
    pub fn init() -> Self {
        Self {
            fbuffer: vec![0; WIDTH * HEIGHT],
            window: Rc::new(Window::new(
                    "Gameboy LCD",
                    WIDTH,
                    HEIGHT,
                    WindowOptions::default(),
                ).unwrap_or_else(|e| {
                    panic!("{}", e);
                })
            ),

            keypad: 0xff,

            cycle: 0,

            vram_bank: 0,
            vram_banks: vec![[0; VRAM_BANK_SIZE]; VRAM_BANK_NUM],

            interrupt: 0,

            ldcd: 0,
            stat: 0,

            scy: 0,
            scx: 0,

            ly: 0,
            lyc: 0, 

            bgp: 0,

            obp0: 0,
            obp1: 0,

            wy: 0,
            wx: 0,
        }
    }

}

impl GPU {
    pub fn step(&mut self, cycles: usize) {
        self.update_keypad();

        self.cycle += cycles;

        if(self.cycle > 6666) {
            self.ly += 1; 
            self.cycle = 0;
        }

        if self.ly >= 153 {
            self.ly = 0;
            
            self.interrupt |= 0x01;

            self.update_tile_map();
            if let Some(win) = Rc::get_mut(&mut self.window) {
                win.update_with_buffer(&self.fbuffer, WIDTH, HEIGHT).unwrap();
            }
        }
    }

    pub fn get_interrupt(&mut self) -> u8 {
        let ret = self.interrupt;
        self.interrupt = 0;
        ret
    }
}

impl GPU {

    // Get and set bank location
    pub fn get_bank(&self) -> u8 {
        self.vram_bank
    }

    pub fn set_bank(&mut self, b: u8) {
        self.vram_bank = b;
    }

    // Read and Write to VRAM
    pub fn read_vram_byte(&self, idx: u16) -> u8 {
        self.vram_banks[self.vram_bank as usize][idx as usize]
    }

    pub fn write_vram_byte(&mut self, idx: u16, val: u8) {
        self.vram_banks[self.vram_bank as usize][idx as usize] = val;
    }
}

impl GPU {

    pub fn read_io_byte(&self, idx: u16) -> u8 {
        match idx {
            0xff00 => self.keypad, 

            0xff40 => self.ldcd,
            0xff41 => self.stat,
            0xff42 => self.scy,
            0xff43 => self.scx,
            0xff44 => self.ly,
            0xff45 => self.lyc,

            0xff47 => self.bgp,
            0xff48 => self.obp0,
            0xff49 => self.obp1,

            0xff4a => self.wy,
            0xff4b => self.wx,

            0xff4f => self.get_bank(),
            
            _ => {
                //dprintln!("Unhandled GPU Read from Address [{:#04x?}]", idx);
                0
            }
        }
    }

    pub fn write_io_byte(&mut self, idx: u16, val: u8) {
        match idx {
            0xff00 => self.keypad = (self.keypad & 0xcf) | (val & 0x30),

            0xff40 => self.ldcd = (self.keypad & 0x07) | (val & 0xf8),
            0xff41 => self.stat = val,
            0xff42 => self.scy = val,
            0xff43 => self.scx = val,
            0xff44 => self.ly = 0,
            0xff45 => self.lyc = val,

            0xff47 => self.bgp = val,
            0xff48 => self.obp0 = val,
            0xff49 => self.obp1 = val,

            0xff4a => self.wy = val,
            0xff4b => self.wx = val,

            0xff4f => self.set_bank(val),

            _ => {
                println!("Unhandled GPU Write from Address [{:#04x?}] [{:#02x?}]", idx, val);
            }
        }
    }
}

impl GPU {

    fn update_keypad(&mut self) {
        self.keypad = 0x00;
        if self.window.as_ref().is_key_down(Key::Right) { self.keypad |= 1 << 0; }
        if self.window.as_ref().is_key_down(Key::Left)  { self.keypad |= 1 << 1; }
        if self.window.as_ref().is_key_down(Key::Up)    { self.keypad |= 1 << 2; }
        if self.window.as_ref().is_key_down(Key::Down)  { self.keypad |= 1 << 3; }
        if self.window.as_ref().is_key_down(Key::A)     { self.keypad |= 1 << 4; }
        if self.window.as_ref().is_key_down(Key::B)     { self.keypad |= 1 << 5; }
        if self.window.as_ref().is_key_down(Key::Z)     { self.keypad |= 1 << 6; }
        if self.window.as_ref().is_key_down(Key::X)     { self.keypad |= 1 << 7; }

        if self.keypad != 0 {
            self.interrupt |= 1 << 4;
        }
    }

}

impl GPU {

    fn get_pixel(&self, x: usize, y: usize) -> u32 {
        self.fbuffer[(y % HEIGHT) * WIDTH + (x % WIDTH)]
    }

    fn set_pixel(&mut self, x: usize, y: usize, val: u32) {
        self.fbuffer[(y % HEIGHT) * WIDTH + (x % WIDTH)] = val;
    }
    
    fn write_tile(&mut self, xpos: usize, ypos: usize, mut tile_addr: u16) {

        for y in 0..8 {
            let line = (
                (self.read_vram_byte(tile_addr + 0) as u16) << 8 |
                (self.read_vram_byte(tile_addr + 1) as u16) << 0
            );

            tile_addr += 2;
            
            for x in 0..8 {
                let bits = (((line >> (2 * x)) & 0x03) as u32) * 0x0f;
                let color = (bits << 0) | (bits << 8) | (bits << 16) | (bits << 24);
                self.set_pixel(x + xpos, y + ypos, color);
            }
        }
    }

    fn update_tile_map(&mut self) {
        if (self.ldcd & (1 << 6)) == 0 {
            for tile in 0x000..0x400 {
                let tile_id = self.read_vram_byte(tile + 0x1c00) as u8 as u16;
                let tile_addr = 0x0000 + 32 * tile_id;
                let x = (tile * 8) as usize;
                let y = ((tile * 8 / (WIDTH as u16)) * 8) as usize;

                self.write_tile(x, y, tile_addr);
            }
        } else {
            for tile in 0x000..0x400 {
                let tile_id = self.read_vram_byte(tile + 0x1800) as i8 as i32;
                let tile_addr = ((0x1000 as i32) + (32 * tile_id)) as u16;
                let x = (tile * 8) as usize;
                let y = ((tile * 8 / (WIDTH as u16)) * 8) as usize;

                self.write_tile(x, y, tile_addr);
            }

        }
    }
}