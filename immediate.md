# Immediate TODO

## 1. Fix compilation (broken right now)

The old `src/memory/mod.rs` was deleted but code still references it:

- `src/emulator/mod.rs` — imports `crate::memory::{init_ram, RAM_SIZE}`, references undefined `bus` variable
- `src/instructions/mod.rs` — imports `crate::memory::load_*/store_*`, takes `mem: &mut [u8]`
- `src/main.rs` — references `emu.mem` which doesn't exist

## 2. Implement Bus read/write

`src/bus/mod.rs` has the struct but an empty `impl`. Add:

```rust
impl Bus {
    pub fn read8(&self, addr: u64) -> u8 { ... }
    pub fn read16(&self, addr: u64) -> u16 { ... }
    pub fn read32(&self, addr: u64) -> u32 { ... }
    pub fn read64(&self, addr: u64) -> u64 { ... }

    pub fn write8(&mut self, addr: u64, val: u8) { ... }
    pub fn write16(&mut self, addr: u64, val: u16) { ... }
    pub fn write32(&mut self, addr: u64, val: u32) { ... }
    pub fn write64(&mut self, addr: u64, val: u64) { ... }
}
```

Each method dispatches by address range (RAM at `0x8000_0000`, UART, PLIC, CLINT at their respective addresses).

## 3. Move instructions into `impl Cpu`, delete `src/instructions/mod.rs`

Instructions are executed by the CPU, not the emulator. Move all instruction handlers into `src/cpu/mod.rs` as private methods on `Cpu`. `Emulator::step()` just fetches and hands off:

```rust
// src/emulator/mod.rs
impl Emulator {
    pub fn step(&mut self) {
        let instruction = self.bus.read32(self.cpu.pc);
        self.cpu.execute(instruction, &mut self.bus);
    }
}

// src/cpu/mod.rs
impl Cpu {
    pub fn execute(&mut self, instruction: u32, bus: &mut Bus) {
        match instruction & 0x7F {
            0b0110011 => self.ins_r(instruction),
            0b0010011 => self.ins_i(instruction),
            0b0000011 => self.ins_load(instruction, bus),
            0b0100011 => self.ins_store(instruction, bus),
            0b1100011 => self.ins_b(instruction),
            0b1101111 => self.ins_j(instruction),
            0b0110111 | 0b0010111 => self.ins_u(instruction),
            _ => panic!("unimplemented opcode: {:#09b}", instruction & 0x7F),
        }
    }

    fn ins_r(&mut self, instruction: u32) { ... }
    fn ins_load(&mut self, instruction: u32, bus: &mut Bus) { ... }
    fn ins_store(&mut self, instruction: u32, bus: &mut Bus) { ... }
}
```

## 4. Complete the MMU

`src/mmu/mod.rs` always returns `0`. Implement the Sv39 page table walk reading PTEs from RAM via the bus.

## 5. Populate peripherals

`bus/uart.rs`, `bus/plic.rs`, `bus/clint.rs` are empty. UART first — it enables console output.

## 6. CSR instructions + privilege

No SYSTEM opcode handler exists yet (CSR read/write, ECALL, EBREAK). Needed for any real software.

---

## Target Structure

```
Emulator                   (src/emulator/mod.rs)
├── step()                 — fetch from bus, hand off to cpu
│
├── cpu: Cpu               (src/cpu/mod.rs)
│   ├── gpr: [u64; 32]
│   ├── csr: [u64; 4096]
│   ├── pc: u64
│   ├── privilege: u64
│   ├── execute()          — opcode dispatch
│   └── ins_r/i/s/b/j/u() — instruction handlers (private &mut self methods)
│
└── bus: Bus               (src/bus/mod.rs)
    ├── read8/16/32/64()
    ├── write8/16/32/64()
    │
    ├── ram: Ram           (src/bus/ram.rs)
    ├── uart: Uart         (src/bus/uart.rs)
    ├── plic: Plic         (src/bus/plic.rs)
    └── clint: Clint       (src/bus/clint.rs)
```

`main.rs` becomes:

```rust
fn main() {
    let mut emu = Emulator::new();
    emu.load_program("program.bin");
    loop {
        emu.step();
    }
}
```
