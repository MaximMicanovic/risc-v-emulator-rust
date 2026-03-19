use crate::cpu::CpuData;
use crate::memory::{init_ram, RAM_SIZE};

pub struct Emulator {
    pub cpu: CpuData,
    pub mem: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        let mut cpu = CpuData::new();
        // STACKPOINTER INIT
        cpu.gpr[2] = RAM_SIZE as u64;
        Emulator {
            cpu,
            mem: init_ram(),
        }
    }
}
