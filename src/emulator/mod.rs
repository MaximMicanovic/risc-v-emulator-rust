use crate::cpu::Cpu;
use crate::bus::Bus;

pub struct Emulator {
    pub cpu: Cpu,
    pub bus: Bus,
}

impl Emulator {
    pub fn new() -> Self {
        let mut cpu = Cpu::new();
        let mut bus = Bus::new();
        // Stackpointer init

        Emulator {
            cpu,
            bus,
        }
    }
}
