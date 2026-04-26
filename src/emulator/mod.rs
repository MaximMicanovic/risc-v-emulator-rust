use crate::cpu::{Mmu, Cpu};
use crate::bus::Bus;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Emulator {
    pub fn new() -> Self {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        // The mmu needs the bus for the
        let mut mmu = Mmu::New();
        // Stackpointer init

        Emulator {
            cpu,
            bus,
        }
    }
}
