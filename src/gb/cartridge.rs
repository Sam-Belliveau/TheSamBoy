use std::fs::File;
use std::io::Read;

use std::vec::Vec;

const CARTRIDGE_BANK_NUM: usize = 256;
const CARTRIDGE_BANK_SIZE: usize = 0x4000;
const CARTRIDGE_EXT_RAM_SIZE: usize = 0x2000;

type ROMBank = [u8; CARTRIDGE_BANK_SIZE];
type ExtRAM = Vec<u8>;

#[derive(Clone)]
pub struct Cartridge {
    // current ROM Bank
    rom_bank : u8, 

    // vector of rom banks due to large memory size
    rom_banks : Vec<ROMBank>,

    // RAM Stored on cartridge
    ext_ram : ExtRAM,
}

impl Cartridge {

    // Create Cartridge
    pub fn load(cartridge: &mut File) -> Self{
        let mut o = Self { 
            rom_bank: 1,
            rom_banks: vec![[0; CARTRIDGE_BANK_SIZE]; CARTRIDGE_BANK_NUM],

            ext_ram: vec![0; CARTRIDGE_EXT_RAM_SIZE]
        };

        for b in o.rom_banks.iter_mut() {
            let i = cartridge.read(b).expect("Error Reading Rom!");

            if i <= 0 {
                break;
            }
        }

        o
    }

    // Get and set bank location
    pub fn get_bank(&self) -> u8 {
        self.rom_bank
    }

    pub fn set_bank(&mut self, b: u8) {
        self.rom_bank = b;
    }

    // Read bytes from Cartridge
    pub fn read_byte(&self, idx: u16) -> u8 {
        self.rom_banks[0][idx as usize]
    }

    pub fn read_bank_byte(&self, idx: u16) -> u8 {
        self.rom_banks[self.rom_bank as usize][idx as usize]
    }

    // Cartridge Ram
    pub fn read_ram_byte(&self, idx: u16) -> u8 {
        self.ext_ram[idx as usize]
    }

    pub fn write_ram_byte(&mut self, idx: u16, val: u8) {
        self.ext_ram[idx as usize] = val;
    }

    // Cartridge Information
    pub fn get_name(&self) -> String {
        String::from(
            std::str::from_utf8(&self.rom_banks[0][0x0134..=0x0142])
                .expect("Invalid Cartridge Name! [is this a gb rom?]")
        )
    }
}
