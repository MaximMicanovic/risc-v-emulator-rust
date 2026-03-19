use crate::cpu::CpuData;

#[allow(dead_code)]
const PAGE_SIZE: u64 = 4096;
#[allow(dead_code)]
const PTE_V: u64 = 1;
#[allow(dead_code)]
const PTE_R: u64 = 2;
#[allow(dead_code)]
const PTE_W: u64 = 4;
#[allow(dead_code)]
const PTE_X: u64 = 8;

pub fn mmu_translate(cpu: &mut CpuData, vaddr: u64, _access: i32) -> u64 {
    if ((cpu.satp >> 60) & 0xF) == 0 {
        return vaddr;
    }

    let root_ppn = cpu.satp & ((1u64 << 44) - 1);
    let _table = root_ppn * PAGE_SIZE;

    let _vpn = [
        (vaddr >> 12) & 0x1FF,
        (vaddr >> 21) & 0x1FF,
        (vaddr >> 30) & 0x1FF,
    ];

    0
}
