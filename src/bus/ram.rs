
pub const RAM_SIZE: u64 = 128 * 1024 * 1024;
pub const RAM_BASE: u64 = 0x8000_0000;
pub const RAM_OFFSET: u64 = RAM_BASE + RAM_SIZE;

pub struct Ram{
    pub ram: Vec<u8>,
}

impl Ram{
    
    pub fn new() -> Self{
        Ram{ ram: vec![0u8; RAM_SIZE as usize] }
    }

    pub fn read8(&mut self, addr: u64) -> u8 {
        self.ram[addr as usize]
    }
    
    pub fn write8(&mut self, addr: u64, val: u64) {
        self.ram[addr as usize] = val as u8;
    }
}
