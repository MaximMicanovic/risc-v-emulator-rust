use crate::cpu::Cpu;

const PAGE_SIZE: u64 = 4096;
const PTE_V: u64 = 1;
const PTE_R: u64 = 2;
const PTE_W: u64 = 4;
const PTE_X: u64 = 8;

pub struct Mmu;

impl Mmu{
    pub fn mmu_translate(satp: u64, vaddr: u64, _access: i32) -> u64 {
        if ((satp >> 60) & 0xF) == 0 {
            return vaddr;
        }

        let root_ppn = satp & ((1u64 << 44) - 1);
        let _table = root_ppn * PAGE_SIZE;
            
        let _vpn = [
            (vaddr >> 12) & 0x1FF,
            (vaddr >> 21) & 0x1FF,
            (vaddr >> 30) & 0x1FF,
        ];
        
        0
    }
}
