use core::arch::asm;

#[repr(align(4096))]
pub struct PageTable {
    pub entries: [u64;512]
}
static mut L0_TABLE: PageTable = PageTable {
    entries: [0; 512],
};

static mut L1_TABLE: PageTable = PageTable {
    entries: [0; 512],
};

static mut L2_TABLE: PageTable = PageTable {
    entries: [0; 512],
};

static mut L3_TABLE: PageTable = PageTable {
    entries: [0; 512],
};

const ENTRY_VALID: u64 = 1 << 0;
const ENTRY_TABLE: u64 = 1 << 1;

const ENTRY_AF: u64 = 1 << 10;
const ENTRY_SH_INNER: u64 = 3 << 8;

const ENTRY_AP_RW: u64 = 0 << 6;
const ENTRY_ATTR_INDEX: u64 = 0 << 2;

pub unsafe fn init_identity_map() {
    // connect page table levels

    L0_TABLE.entries[0] =
        (&L1_TABLE as *const _ as u64) | ENTRY_VALID | ENTRY_TABLE;

    L1_TABLE.entries[0] =
        (&L2_TABLE as *const _ as u64) | ENTRY_VALID | ENTRY_TABLE;

    L2_TABLE.entries[0] =
        (&L3_TABLE as *const _ as u64) | ENTRY_VALID | ENTRY_TABLE;

    // map first 2MB region with 4KB pages

    for i in 0..512 {
        let addr = (i as u64) << 12;

        L3_TABLE.entries[i] =
            addr
            | ENTRY_VALID
            | ENTRY_AF
            | ENTRY_SH_INNER
            | ENTRY_AP_RW
            | ENTRY_ATTR_INDEX;
    }
}

pub unsafe fn  enable_mmu() {
    let l0 = &L0_TABLE as *const _ as u64;

    // Memory attributes
    asm!(
        "msr mair_el1, {0}",
        in(reg) 0xff
    );

    // Translation control register
    asm!(
        "msr tcr_el1, {0}",
        in(reg) (16 << 0) | (16 << 16)
    );

    // Set translation table base
    asm!(
        "msr ttbr0_el1, {0}",
        in(reg) l0
    );

    asm!("dsb sy");
    asm!("isb");

    // Enable MMU
    let mut sctlr: u64;

    asm!(
        "mrs {0}, sctlr_el1",
        out(reg) sctlr
    );

    sctlr |= 1; // enable MMU

    asm!(
        "msr sctlr_el1, {0}",
        in(reg) sctlr
    );

    asm!("isb");
}