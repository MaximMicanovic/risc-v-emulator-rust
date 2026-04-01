# MMU — What's Missing

Current state: `mmu_translate` returns `0` unconditionally after computing some values it never uses.

---

## 1. Page table walk (critical)

The function extracts `root_ppn` and `vpn[0..2]` but never actually walks the page table.

A Sv39 walk requires:
- Start at `root_ppn * PAGE_SIZE`
- For each VPN level (2 → 1 → 0):
  - Read the 8-byte PTE at `table + vpn[level] * 8` from physical memory
  - Check `PTE_V`; raise a page fault if not set
  - If it's a leaf (R or X bit set), stop and form the physical address
  - Otherwise descend: `table = pte_ppn * PAGE_SIZE`
- Form the final physical address from the leaf PTE's PPN + page offset

This requires access to physical memory (the `Bus`), which the function currently doesn't receive.

## 2. No bus/memory access

`mmu_translate` has no way to read PTEs. It needs a reference to `Bus` (or at minimum `RAM`) passed in so it can do `bus.read64(pte_addr)`.

## 3. Access type is unused (`_access: i32`)

The `_access` parameter (read / write / execute) is ignored. It's needed to:
- Select the correct permission bit to check (`PTE_R`, `PTE_W`, `PTE_X`)
- Generate the right page fault cause code (12 = fetch, 13 = load, 15 = store)

Consider replacing the `i32` with an enum (`Fetch`, `Load`, `Store`).

## 4. No page fault / trap generation

On any fault (invalid PTE, permission denied, misaligned superpage), the function should signal a trap rather than silently returning `0`. This likely means returning a `Result<u64, TrapCause>` or integrating with whatever trap mechanism the CPU uses.

## 5. Sv39 only — mode check is incomplete

The check `(satp >> 60) & 0xF == 0` correctly detects bare mode, but values 1–7 and 9–15 are reserved; only `8` (Sv39) and `9` (Sv48) are valid non-bare modes. Currently any non-zero mode falls through as if it were Sv39.

## 6. Superpage support missing

A leaf PTE found at level 2 or 1 is a superpage (1 GiB / 2 MiB). The physical address formation must zero the lower VPN bits that are "inherited" from the virtual address, and the PTE PPN alignment must be validated.

## 7. PTE bits A/D not handled

RISC-V requires the Accessed (A) bit to be set on any access and the Dirty (D) bit on writes. The implementation should either set these bits or raise a page fault if they are clear, depending on whether the hardware-managed or software-managed A/D model is chosen.

## 8. ASID not used

`satp[59:44]` carries the ASID. It is extracted as part of `root_ppn` but masked off — that's fine — however there is no TLB, so ASID is irrelevant until a TLB is added. Worth noting for later.

---

## Summary

| Area | Status |
|---|---|
| Sv39 page table walk | Not implemented (always returns 0) |
| Physical memory access | Missing (no Bus parameter) |
| Access-type permission check | Missing |
| Page fault / trap signalling | Missing |
| Superpage handling | Missing |
| PTE A/D bit management | Missing |
| Mode validation (non-Sv39 modes) | Incomplete |
