pub const RAM_SIZE: usize = 4 * 1024 * 1024;

// LOADING INTO CPU REGISTER
pub fn load_byte(mem: &[u8], addr: u32) -> u32 {
    (mem[addr as usize] as i8 as i32) as u32
}

pub fn load_h(mem: &[u8], addr: u32) -> u32 {
    let val = (mem[addr as usize] as u16) | ((mem[addr as usize + 1] as u16) << 8);
    (val as i16 as i32) as u32
}

pub fn load_w(mem: &[u8], addr: u32) -> u32 {
    let val = (mem[addr as usize] as u32)
        | ((mem[addr as usize + 1] as u32) << 8)
        | ((mem[addr as usize + 2] as u32) << 16)
        | ((mem[addr as usize + 3] as u32) << 24);
    val as i32 as u32
}

pub fn load_ubyte(mem: &[u8], addr: u32) -> u32 {
    mem[addr as usize] as u32
}

pub fn load_uh(mem: &[u8], addr: u32) -> u32 {
    let val = (mem[addr as usize] as u16) | ((mem[addr as usize + 1] as u16) << 8);
    val as u32
}

// STORING INTO MEMORY
pub fn store_byte(mem: &mut [u8], addr: u32, val: u32) {
    mem[addr as usize] = val as u8;
}

pub fn store_h(mem: &mut [u8], addr: u32, val: u32) {
    mem[addr as usize] = val as u8;
    mem[addr as usize + 1] = (val >> 8) as u8;
}

pub fn store_w(mem: &mut [u8], addr: u32, val: u32) {
    mem[addr as usize] = val as u8;
    mem[addr as usize + 1] = (val >> 8) as u8;
    mem[addr as usize + 2] = (val >> 16) as u8;
    mem[addr as usize + 3] = (val >> 24) as u8;
}

// Returning RAM Vec
pub fn init_ram() -> Vec<u8> {
    vec![0u8; RAM_SIZE]
}
