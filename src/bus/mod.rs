pub mod ram;
pub mod clint;
pub mod plic;
pub mod uart;

use crate::bus::ram::{RAM_BASE, RAM_OFFSET};
use crate::bus::uart::{UART_BASE, UART_OFFSET};

use self::uart::Uart;
use self::ram::Ram;
// use self::plic::Plic;

pub struct Bus {
    pub ram: Ram,
    pub uart: Uart,
    // pub plic: Plic,
    // pub clint: Clint,
}

impl Bus{
    pub fn new() -> Self{
        let mut ram: Ram = Ram::new();
        let uart: Uart = Uart;

        Bus{
            ram,
            uart,
        }
    }


    pub fn read8(&mut self, addr: u64) -> u8 {
        match addr {
            // TODO
            //0x0000_0000..=0x0000_FFFF => // boot rom,
            //0x0200_0000..=0x0200_FFFF => // clint,
            //0x0C00_0000..=0x0FFF_FFFF => // plic,
            UART_BASE..=UART_OFFSET => return self.uart.read8(addr - UART_BASE),
            RAM_BASE..=RAM_OFFSET => return self.ram.read8(addr - RAM_BASE),
            _ => panic!("unmapped address: {:#010x}", addr),
        }
    }
    pub fn read16(){}

    pub fn read32(){}

    pub fn read64(){}
}
