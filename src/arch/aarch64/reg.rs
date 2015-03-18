def_reg!(
    mpidr_el1, u64, ro,
    AFF0(7, 0),
    );

def_reg!(
    ttbr0_el1, u64, rw,
    BADDR(47, 0),
    ASID(63, 48),
    );

def_reg!(
    daif, u64, rw,
    F(0, 0),
    I(1, 1),
    A(2, 2),
    D(3, 3),
    );

