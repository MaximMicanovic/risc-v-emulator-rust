use std::io::Read;

pub struct Uart;
pub const UART_BASE: u64 = 0x1000_0000;
pub const UART_OFFSET: u64 = 0x1000_00FF;

impl Uart{
    pub fn write8(&self, reg: u64, val: u8){
        match reg{
            0x00 => print!("{}", val as char),
            _ => {}
        }
    }

    pub fn read8(&self, reg: u64) -> u8 {
        match reg{
            0x00 => { 
                let mut buf = [0u8; 1];
                std::io::stdin().read_exact(&mut buf);
                buf[0]
            }
            0x05 => 0x60,
            _ => 0,
        }
    }
}
