
const VRAM_BANK_NUM : usize = 2;
const VRAM_BANK_SIZE : usize = 0x2000;

type VRAMBank = [u8; VRAM_BANK_SIZE];

#[derive(Clone)]
pub struct GPU {

    vram_bank: u8,
    vram_banks: Vec<VRAMBank>
}

impl GPU {

    // Make new GPU
    pub fn init() -> Self {
        GPU {
            vram_bank: 0,
            vram_banks: vec![[0; VRAM_BANK_SIZE]; VRAM_BANK_NUM],
        }
    }

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