def_bitfields!(u64,
               PTE_TYPE(1, 0),
               PTE_LATTRS(11, 2),
               PTE_ADDR(47, 12),
               PTE_HATTRS(63, 52),
               PTE_XN(54, 54),
               PTE_NG(11, 11),
               PTE_SH(9, 8),
               PTE_AP(7, 6),
               PTE_NS(5, 5),
               PTE_ATTRINDX(4, 2),
               );

pub const PTE_TYPE_INVALID : u64 = 0x0;
pub const PTE_TYPE_BLOCK : u64 = 0x1;
pub const PTE_TYPE_TABLE : u64 = 0x3;

pub const PTE_AP_KERNEL : u64 = 0x0;
pub const PTE_AP_RW: u64 = 0x1;
pub const PTE_AP_KERNEL_RO: u64 = 0x2;
pub const PTE_AP_RO: u64 = 0x3;
