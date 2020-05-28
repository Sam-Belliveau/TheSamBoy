use std::vec::Vec;

const RAM_BANK_NUM: usize = 8;
const RAM_BANK_SIZE: usize = 0x4000;

type RAMBank = [u8; RAM_BANK_SIZE];

#[derive(Clone)]
pub struct WorkRAM {
    // current work ram bank
    ram_bank: u8,

    // ram banks stored in vector due to large memory size
    ram_banks: Vec<RAMBank>,
}

impl WorkRAM {

    pub fn init() -> Self {
        Self {
            ram_bank: 1,
            ram_banks: vec![[0; RAM_BANK_SIZE]; RAM_BANK_NUM],
        }
    }

}

impl WorkRAM {
    // Get and set bank location
    pub fn get_bank(&self) -> u8 {
        self.ram_bank
    }

    pub fn set_bank(&mut self, b: u8) {
        self.ram_bank = b;
    }

    // Read and Write to RAM
    pub fn read_byte(&self, idx: u16) -> u8 {
        self.ram_banks[0][idx as usize]
    }

    pub fn write_byte(&mut self, idx: u16, val: u8) {
        self.ram_banks[0][idx as usize] = val;
    }

    // Read and Write to RAM Bank
    pub fn read_bank_byte(&self, idx: u16) -> u8 {
        self.ram_banks[self.ram_bank as usize][idx as usize]
    }

    pub fn write_bank_byte(&mut self, idx: u16, val: u8) {
        self.ram_banks[self.ram_bank as usize][idx as usize] = val;
    }
}