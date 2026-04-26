use crate::cpu::Cpu;
use crate::bus::Bus;

const PAGE_SIZE: u64 = 4096;
const PTE_V: u64 = 1;
const PTE_R: u64 = 2;
const PTE_W: u64 = 4;
const PTE_X: u64 = 8;

pub enum AccessType{
    Fetch,
    Load,
    Store,
}

pub fn mmu_translate(cpu: &Cpu, bus: &Bus, vaddr: u64, access: AccessType) -> Results<u64, u64> {
        let mode = (cpu.satp >> 60) & 0xF;
        // Priveledge mode & Bare access
        if (mode == 0 || cpu.priveledge == 3) {
            return Ok(vaddr);
        }

        // Making sure its Sv39 3 level page walk
        assert!(mode == 8, "unsupported stap mode");

        // First 44 bits are [0, 43]
        let root_ppn = cpu.satp & ((1u64 << 44) - 1);
        let table = root_ppn * PAGE_SIZE;
            
        let vpn = [
            (vaddr >> 12) & 0x1FF,
            (vaddr >> 21) & 0x1FF,
            (vaddr >> 30) & 0x1FF,
        ];
        
        // The walk happening here
        for level in (0..3).rev() {
            let pte_addr = table + vpn[level] * 8;
            let pte = bus.read64(pte_addr);

            // V bit not set page entry dosent exists
            if pte & PTE_V == 0 {
                return Err(match access{
                    AccessType::Fetch => 12,
                    AccessType::Load  => 13,
                    AccessType::Store => 15,
                });
            }

            // This is a leaf
            if pte & (PTE_R | PTE_X) != 0 {
                match access {
                    AccessType::Fetch => if pte & PTE_X == 0 { return Err(12); },
                    AccessType::Load => if pte & PTE_R == 0 { return Err(13); },
                    AccessType::Store => if pte & PTE_W == 0 { return Err(15); },
                }
            }
        }
        0
}
