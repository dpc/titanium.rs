
pub mod pte {
    def_bitfields!(u64,
                   TYPE(1, 0),
                   LATTRS(11, 2),
                   ADDR(47, 12),
                   HATTRS(63, 52),
                   XN(54, 54),
                   NG(11, 11),
                   SH(9, 8),
                   AP(7, 6),
                   NS(5, 5),
                   ATTRINDX(4, 2),
                   );

    pub const TYPE_INVALID : u64 = 0x0;
    pub const TYPE_BLOCK : u64 = 0x1;
    pub const TYPE_TABLE : u64 = 0x3;

    pub const AP_KERNEL : u64 = 0x0;
    pub const AP_RW: u64 = 0x1;
    pub const AP_KERNEL_RO: u64 = 0x2;
    pub const AP_RO: u64 = 0x3;
}
