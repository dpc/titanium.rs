def_reg!(
    mpidr_el1, u64, ro,
    AFF0(7, 0),
    );

def_reg!(
    ttbr0_el1, u64, rw,
    BADDR(47, 0),
    ASID(63, 48),
    );

