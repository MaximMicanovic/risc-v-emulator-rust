#[repr(u64)]
pub enum CpuPriv {
    U = 0,
    S = 1,
    M = 3,
}

pub enum CpuStatus {
    Ok = 0,
    Halt,
    Trap,
    ErrFetchFault,
    ErrIllegalInsn,
    ErrMisaligned,
}

pub struct CpuData {
    pub gpr: [u64; 32],
    pub csr: [u64; 4096],
    pub pc: u64,
    pub privilege: u64,
    pub satp: u64,
}

impl CpuData {
    pub fn new() -> Self {
        CpuData {
            gpr: [0; 32],
            csr: [0; 4096],
            pc: 0,
            privilege: 0,
            satp: 0,
        }
    }
}
