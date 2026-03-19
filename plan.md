# Plan: Boot Linux in the Terminal

## Architecture

### Modules

| Module | Owns | Responsibility |
|---|---|---|
| `main.rs` | nothing | arg parsing, load kernel/DTB from disk, call `emu.run()` |
| `emulator` | `CpuData`, `Bus`, `Sbi` | main `step()` / `run()` loop; fetch, trap dispatch, interrupt check, `instret` increment |
| `cpu` | `gpr`, `fpr`, `csr`, `pc`, `privilege`, `reservation` | all CPU state; CSR read/write with WARL enforcement and privilege checks |
| `instructions` | nothing | decode and execute; signals traps back to emulator via return value; calls `mmu::access()` for all memory |
| `mmu` | nothing | virtual→physical translation; bypasses in M-mode; walks Sv39 page tables via bus; raises page faults |
| `bus` | `Vec<u8>` (RAM), all device structs, Boot ROM | MMIO address dispatch; every physical memory read/write goes here |
| `sbi` | nothing | intercepts `ecall` from S-mode; emulates SBI extensions (TIME, IPI, HSM, RFENCE, console) |
| `devices/uart` | UART register state | MMIO read/write; writes chars to stdout; signals RX interrupt to PLIC |
| `devices/clint` | `mtime`, `mtimecmp` | increments `mtime` each step; raises machine timer interrupt when `mtime >= mtimecmp` |
| `devices/plic` | interrupt priority/enable/pending state | routes device interrupts to CPU `mip.SEIP` |

---

### Full Communication Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                           main.rs                               │
│   - parse CLI args (kernel path, initramfs path)                │
│   - read kernel Image + DTB from disk                           │
│   - call Emulator::new(kernel, dtb)                             │
│   - call emu.run()                                              │
└──────────────────────────────┬──────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                          Emulator                               │
│                                                                 │
│  step():                                                        │
│    1. translate PC through MMU → physical fetch address         │
│    2. read 2 bytes from bus → check if C-extension (bits[1:0])  │
│       - if compressed: decode 16-bit, advance PC by 2           │
│       - if normal:     read 2 more bytes, advance PC by 4       │
│    3. call instructions::execute(cpu, bus) → Result<(), Trap>   │
│    4. if Trap returned:                                         │
│         if ecall from S-mode → hand to sbi::handle()           │
│         else → cpu::take_trap(cause, tval)                      │
│    5. clint::tick() → check mtime >= mtimecmp → set mip.MTIP   │
│    6. plic::check() → set mip.SEIP if interrupt pending        │
│    7. check mstatus + mie + mip → take interrupt if enabled     │
│    8. cpu.csr[instret] += 1                                     │
│                                                                 │
│  ┌──────────────────────────┐                                   │
│  │          cpu/            │                                   │
│  │                          │                                   │
│  │  gpr:  [u64; 32]         │  ← instructions reads/writes     │
│  │  fpr:  [f64; 32]         │    registers directly            │
│  │  csr:  [u64; 4096]       │                                   │
│  │  pc:   u64               │  ← emulator updates PC           │
│  │  priv: u64               │  ← trap logic changes privilege  │
│  │  resv: Option<u64>       │  ← LR/SC reservation address     │
│  │                          │                                   │
│  │  fn csr_read(addr)       │  ← enforces privilege + WARL     │
│  │  fn csr_write(addr, val) │                                   │
│  │  fn take_trap(cause,tval)│  ← writes mepc/mcause/mtvec etc  │
│  └──────────────────────────┘                                   │
│                                                                 │
│  ┌──────────────────────────┐                                   │
│  │      instructions/       │                                   │
│  │                          │                                   │
│  │  execute(cpu, bus)       │                                   │
│  │    → Result<(), Trap>    │  ← returns trap, never panics     │
│  │                          │  on valid but unhandled state     │
│  │  all memory access:      │                                   │
│  │    mmu::access(cpu, bus, │                                   │
│  │      vaddr, AccessType)  │                                   │
│  │    → Result<u64, Trap>   │                                   │
│  └──────────────────────────┘                                   │
│                                                                 │
│  ┌──────────────────────────┐                                   │
│  │          sbi/            │                                   │
│  │                          │                                   │
│  │  handle(cpu, bus)        │  ← called on ecall from S-mode   │
│  │                          │                                   │
│  │  EXT_BASE   → probe      │                                   │
│  │  EXT_TIME   → mtimecmp   │  ← writes clint via bus          │
│  │  EXT_CONSOLE→ stdout     │                                   │
│  │  EXT_IPI    → no-op      │                                   │
│  │  EXT_RFENCE → no-op      │                                   │
│  │  EXT_HSM    → hart state │                                   │
│  └──────────────────────────┘                                   │
│                                                                 │
└──────────────────────────────┬──────────────────────────────────┘
                               │ all memory access goes through mmu
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                           mmu/                                  │
│                                                                 │
│  access(cpu, bus, vaddr, AccessType) → Result<u64, Trap>        │
│                                                                 │
│  - if cpu.privilege == M-mode: pass vaddr directly to bus       │
│  - if satp.MODE == 0:          pass vaddr directly to bus       │
│  - if satp.MODE == 8 (Sv39):   walk 3-level page table via bus  │
│      VPN[2] → VPN[1] → VPN[0] → PPN + page offset              │
│      check V/R/W/X bits → raise page fault on violation         │
│      set Accessed + Dirty bits in PTE                           │
│      handle superpages at level 1 and 2                         │
│                                                                 │
│  AccessType: Load | Store | Fetch                               │
│  Fault:      LoadPageFault | StorePageFault | InstrPageFault    │
│                                                                 │
└──────────────────────────────┬──────────────────────────────────┘
                               │ physical address
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                           bus/                                  │
│                                                                 │
│  read(paddr, size)  → u64                                       │
│  write(paddr, size, val)                                        │
│                                                                 │
│  address dispatch:                                              │
│                                                                 │
│  0x00001000 – 0x00001FFF  ┌─────────────┐                      │
│                           │  Boot ROM   │ static byte array,   │
│                           │  (in bus)   │ jumps to 0x80200000  │
│                           └─────────────┘                      │
│  0x02000000 – 0x0200FFFF  ┌─────────────┐                      │
│                           │   clint/    │ mtime, mtimecmp,     │
│                           │             │ msip                 │
│                           └─────────────┘                      │
│  0x0C000000 – 0x0FFFFFFF  ┌─────────────┐                      │
│                           │   plic/     │ priority, pending,   │
│                           │             │ enable, claim        │
│                           └─────────────┘                      │
│  0x10000000 – 0x10000FFF  ┌─────────────┐                      │
│                           │   uart/     │ THR→stdout           │
│                           │             │ RBR←stdin            │
│                           └──────┬──────┘                      │
│                                  │ RX interrupt                │
│                                  ▼                             │
│                           ┌─────────────┐                      │
│                           │   plic/     │ sets SEIP in mip     │
│                           └─────────────┘                      │
│  0x80000000 – 0x87FFFFFF  ┌─────────────┐                      │
│                           │  memory/    │ 128MB RAM            │
│                           │  Vec<u8>    │ kernel + initramfs   │
│                           └─────────────┘                      │
└─────────────────────────────────────────────────────────────────┘
```

---

### Trap Flow

```
instruction executes
        │
        ├─ ok ──────────────────────────────► continue
        │
        └─ Trap(cause, tval)
                │
                ├─ ecall from S-mode ────────► sbi::handle(cpu, bus)
                │
                └─ anything else
                        │
                        ├─ medeleg has cause? ──► S-mode trap
                        │    sepc = pc
                        │    scause = cause
                        │    stval = tval
                        │    pc = stvec
                        │    privilege = S
                        │
                        └─ default ─────────────► M-mode trap
                             mepc = pc
                             mcause = cause
                             mtval = tval
                             pc = mtvec
                             privilege = M
```

---

### Fetch Flow (C extension aware)

```
pc
 │
 ▼
mmu::access(Fetch) → paddr
 │
 ▼
bus.read(paddr, 2) → half word
 │
 ├─ bits[1:0] != 0b11 ──► compressed (16-bit)
 │                         decode in instructions::decode_compressed()
 │                         pc += 2
 │
 └─ bits[1:0] == 0b11 ──► normal (32-bit)
                          bus.read(paddr + 2, 2) → upper half
                          combine into full word
                          pc += 4
```

---

### Key Decisions

- **All memory access goes through `mmu::access()`** — including loads and stores from `instructions`, not just fetch. `mmu` bypasses translation in M-mode or when `satp.MODE == 0`, so there is no separate fast path to manage.
- **`instructions` returns `Result<(), Trap>`** — it never directly modifies the PC on a trap or calls trap handling itself. It signals the trap back to the emulator loop which decides where to send it (SBI or `cpu::take_trap`).
- **SBI is its own module** — it is not part of the M-mode trap handler in `cpu`. The emulator loop intercepts `ecall` from S-mode before calling `cpu::take_trap` and routes it to `sbi::handle()` instead.
- **Boot ROM is a static array inside `bus`** — it is small (a few instructions) and never changes, so it does not need its own module.
- **LR/SC reservation lives in `cpu`** — `cpu.reservation: Option<u64>` holds the reserved physical address. Set by `LR`, cleared on `SC` or any other store to that address.
- **`instret` and `cycle` are incremented by the emulator loop** — after every successful `step()`, not inside `instructions`.
- **`clint::tick()` is called every step** — the emulator loop calls it unconditionally; CLINT internally decides whether to fire a timer interrupt based on `mtime` vs `mtimecmp`.
- **Devices are separate submodules** — `devices/uart.rs`, `devices/clint.rs`, `devices/plic.rs` each expose `read(offset, size) -> u64` and `write(offset, size, val)`. The bus strips the base address before calling them.

---

## Current State Summary

- RV32I + M instructions: implemented and tested
- MMU: stub (returns 0, no page table walk)
- CSRs: storage exists, no logic
- Peripherals: none (display is disconnected SDL2 stub)
- Emulator core: skeleton only
- Critical bugs: `cpu->mem` missing from struct, broken `main.c`

---

## Phase 0 — Fix existing bugs

These must be fixed before anything else can be built on top.

1. Add `uint8_t *mem` to `cpudata` struct in `src/cpu/cpu.h`
2. Fix `store_h()` and `store_w()` byte masking in `src/memory/memory.c`
3. Fix `main.c` (wrong register name `cpu.x[2]`, wrong function name `reader()`, undefined `print_instruction()`)
4. Wire `init_RAM()` into the CPU struct on startup
5. Expand RAM from 4MB to at least 128MB (Linux needs room for kernel + initramfs + heap)

---

## Phase 1 — Upgrade to RV64

Linux requires 64-bit RISC-V. The current code uses `uint64_t` for registers already, but the instruction decoder and memory ops are 32-bit oriented.

1. Add 64-bit load/store variants: `LD`, `SD`, `LWU`
2. Add 64-bit arithmetic: `ADDW`, `SUBW`, `SLLW`, `SRLW`, `SRAW`, and their immediate forms (`ADDIW`, `SLLIW`, `SRLIW`, `SRAIW`)
3. Add 64-bit M-extension: `MULW`, `DIVW`, `DIVUW`, `REMW`, `REMUW`
4. Fix sign-extension behavior throughout (RV64 semantics differ from RV32 for word ops)
5. Change PC and address space to 64-bit throughout

---

## Phase 2 — Complete the privileged architecture

This is the biggest chunk. Linux depends heavily on M-mode/S-mode/U-mode separation and trap handling.

### 2a. CSR logic

The CSR array exists but nothing reads/writes it with correct semantics. Need:

- Implement `CSRRW`, `CSRRS`, `CSRRC`, `CSRRWI`, `CSRRSI`, `CSRRCI` instructions
- Enforce read-only bits, WARL fields, privilege-level access restrictions
- Key CSRs to implement correctly:

| CSR | Purpose |
|-----|---------|
| `mstatus` / `sstatus` | Global interrupt enable, privilege tracking |
| `mtvec` / `stvec` | Trap vector base address |
| `mepc` / `sepc` | Exception program counter |
| `mcause` / `scause` | Trap cause code |
| `mtval` / `stval` | Trap value (bad address, bad instruction) |
| `mie` / `sie` | Interrupt enable bits |
| `mip` / `sip` | Interrupt pending bits |
| `mideleg` / `medeleg` | Trap delegation to S-mode |
| `satp` | Page table root + mode |
| `mscratch` / `sscratch` | Scratch registers |
| `mhartid` | Hart (hardware thread) ID |
| `cycle`, `time`, `instret` | Performance counters |

### 2b. Trap and interrupt dispatch

- On exception (bad address, illegal instruction, etc.): save `pc` to `mepc`/`sepc`, write cause to `mcause`/`scause`, jump to `mtvec`/`stvec`
- On interrupt: same, but set MSB of cause
- Implement `MRET` and `SRET` to return from traps (restore PC from `mepc`/`sepc`, restore privilege)
- Implement `ECALL` (generates environment call exception — this is how Linux makes SBI calls and syscalls)
- Implement `EBREAK`
- Implement `WFI` (wait for interrupt — can be a no-op or a `pause`)
- Implement `SFENCE.VMA` (TLB flush — can be a no-op initially)

---

## Phase 3 — Complete virtual memory (Sv39)

The `mmu_translate()` function is a stub. Need a real 3-level page table walk.

1. When `satp.MODE == 8` (Sv39), extract PPN from `satp`
2. Walk 3 levels of page tables in physical memory:
   - Level 2: VPN[2] → PTE at `PPN * 4096 + VPN[2] * 8`
   - Level 1: VPN[1] → PTE at `PPN * 4096 + VPN[1] * 8`
   - Level 0: VPN[0] → PTE at `PPN * 4096 + VPN[0] * 8`
3. Check PTE valid bit; raise page fault if not valid
4. Check R/W/X permissions vs access type; raise page fault on mismatch
5. Handle superpages (when a non-leaf PTE is found at level 1 or 2)
6. Set Accessed and Dirty bits in PTEs as required
7. Raise `LOAD_PAGE_FAULT`, `STORE_PAGE_FAULT`, or `INSTRUCTION_PAGE_FAULT` exceptions on failure
8. Apply translation to every memory access (loads, stores, instruction fetches) when not in M-mode

---

## Phase 4 — A extension (atomics)

Linux uses atomic operations extensively for locking and synchronization. Without these, the kernel won't boot.

- `LR.W` / `LR.D` — load-reserved
- `SC.W` / `SC.D` — store-conditional
- `AMOSWAP`, `AMOADD`, `AMOXOR`, `AMOAND`, `AMOOR`, `AMOMIN`, `AMOMAX`, `AMOMINU`, `AMOMAXU` (both `.W` and `.D` variants)

Since this is a single-hart emulator, LR/SC can be implemented with a simple reservation address flag.

---

## Phase 5 — Core peripherals (MMIO)

Need a memory-mapped I/O dispatch layer. All physical addresses outside DRAM get routed to device handlers.

### Physical memory map (standard RISC-V virt machine layout)

| Address | Device |
|---------|--------|
| `0x0000_1000` | Boot ROM |
| `0x0200_0000` | CLINT (timer + software interrupts) |
| `0x0C00_0000` | PLIC (interrupt controller) |
| `0x1000_0000` | UART 16550 |
| `0x8000_0000` | DRAM base (128MB+) |

### 5a. UART 16550A

Most critical — this is what gives you the terminal. Linux prints to it from very early in boot.

- Implement MMIO registers: `RBR/THR` (receive/transmit), `IER`, `IIR/FCR`, `LCR`, `MCR`, `LSR`, `MSR`
- On write to `THR`: output character to `stdout`
- On read from `RBR`: read character from `stdin` (non-blocking)
- `LSR` bit 5 (THRE) and bit 6 (TEMT): always set (transmitter always ready)
- Raise UART interrupt via PLIC when character received

### 5b. CLINT (Core-Local Interruptor)

- `mtime` register at `0x0200_bff8`: increment on every cycle (or at a fixed rate)
- `mtimecmp` register at `0x0200_4000`: when `mtime >= mtimecmp`, raise machine timer interrupt
- Software interrupt register at `0x0200_0000`

### 5c. PLIC (Platform-Level Interrupt Controller)

- Manages external interrupt routing to harts
- Registers: priority, pending, enable, threshold, claim/complete
- Linux uses this to receive UART interrupts
- Can be simplified initially — implement just enough for UART

---

## Phase 6 — F and D extensions (floating point)

Linux needs basic floating-point support. The kernel itself doesn't use FP heavily but it saves/restores FP state on context switches, and glibc uses FP.

1. Add 32 floating-point registers (`f0`–`f31`) to `cpudata`
2. Add `fcsr` CSR (floating-point control/status)
3. Implement F-extension (single precision): `FLW`, `FSW`, `FADD.S`, `FSUB.S`, `FMUL.S`, `FDIV.S`, `FSQRT.S`, `FMIN.S`, `FMAX.S`, `FCVT.*`, `FMV.*`, `FMADD.S`, `FMSUB.S`, `FNMADD.S`, `FNMSUB.S`, comparisons
4. Implement D-extension (double precision): same set with `.D` suffix
5. Update `mstatus.FS` bits to track FP state (Off/Initial/Clean/Dirty)

---

## Phase 7 — C extension (compressed instructions)

Most Linux binaries compiled for `rv64gc` include compressed 16-bit instructions. Without this, you can't run standard toolchain output.

- Detect 16-bit instructions (when `insn[1:0] != 11`)
- Decode all 3 quadrants (Q0, Q1, Q2) of compressed instructions
- Map each to its equivalent 32-bit form
- Affects instruction fetch logic

---

## Phase 8 — SBI (Supervisor Binary Interface)

Linux communicates with firmware (M-mode) via SBI calls (done via `ecall` from S-mode). You need to either run OpenSBI as a real firmware blob or emulate SBI directly in the emulator.

**Recommended: emulate SBI directly** (intercept `ecall` from S-mode in the M-mode trap handler)

Implement these SBI extensions:

- `sbi_console_putchar` / `sbi_console_getchar` (legacy, but Linux still uses them early)
- `SBI_EXT_BASE` (probe extensions)
- `SBI_EXT_TIME` (set timer — replaces direct `mtimecmp` access from S-mode)
- `SBI_EXT_IPI` (send inter-processor interrupts — no-op for single hart)
- `SBI_EXT_RFENCE` (remote fence — no-op for single hart)
- `SBI_EXT_HSM` (hart state management — needed by recent kernels)

---

## Phase 9 — Boot infrastructure

### 9a. Device Tree Blob (DTB)

Linux needs a device tree describing the hardware. Generate or embed a minimal DTB at boot time and pass its physical address in register `a1`.

Minimum DTB must describe:

- CPU: `riscv`, ISA string (`rv64imafdcsu`), MMU type (`sv39`), `timebase-frequency`
- Memory: base address `0x80000000`, size
- CLINT: reg, compatible
- PLIC: reg, compatible, `riscv,ndev`
- UART: reg, compatible (`ns16550a`), clock-freq, interrupt parent + number
- `chosen` node: `bootargs` (kernel command line, e.g. `"root=/dev/ram rw console=ttyS0"`)

### 9b. Kernel image loading

- Load a compiled Linux kernel (`Image` or `vmlinux`) into DRAM at `0x80200000`
- Load initramfs (BusyBox-based root filesystem) at a higher address
- Set `a0 = hart ID = 0`, `a1 = DTB physical address`
- Set `PC = 0x80200000` (or wherever the kernel entry is)

### 9c. Boot ROM

A small boot ROM at `0x1000` that jumps to `0x80000000` (OpenSBI) or directly to `0x80200000` (kernel) if not running real SBI firmware.

---

## Phase 10 — Emulator core and main loop

Wire everything together in `src/emulator/emulator.c`:

1. Initialize memory with correct size (128MB+)
2. Set up MMIO dispatch: route physical addresses to UART / CLINT / PLIC / RAM
3. Load kernel image and DTB from files (CLI args)
4. Set initial CPU state (PC, privilege = M-mode, registers)
5. Main execution loop:
   - Translate PC via MMU (or bypass in M-mode)
   - Fetch instruction
   - Decode and execute
   - Check for pending interrupts (timer, UART) after each instruction
   - Dispatch interrupts if enabled in `mstatus` and `mie`
   - Handle traps

---

## Implementation order (recommended)

```
Phase 0  → Fix bugs, expand RAM
Phase 1  → RV64 instructions
Phase 2a → CSR read/write instructions
Phase 2b → Trap dispatch, MRET/SRET/ECALL
Phase 3  → Sv39 MMU page table walk
Phase 4  → Atomics
Phase 5a → UART (terminal I/O)
Phase 5b → CLINT (timer interrupts)
Phase 8  → SBI emulation
Phase 9  → DTB + boot loader
Phase 10 → Emulator main loop

[At this point you can attempt to boot Linux and diagnose what's missing]

Phase 5c → PLIC
Phase 6  → Floating point
Phase 7  → Compressed instructions
```

---

## What to test against

- **Kernel:** Build a minimal Linux kernel for `rv64gc` with `CONFIG_SERIAL_8250=y` and a BusyBox initramfs. The RISC-V Linux kernel team's `defconfig` is a good starting point.
- **Reference:** QEMU's `virt` machine (`qemu-system-riscv64 -machine virt`) uses the same memory map described above and is the standard reference implementation.
- **Simpler milestones first:** Before Linux, try booting a bare-metal "Hello World" RISC-V ELF, then try xv6-riscv (a simpler teaching OS), then attempt Linux.
