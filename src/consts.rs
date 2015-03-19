pub const SZ_4KB_SHIFT : usize = 12;
pub const SZ_4KB : usize = 1 << SZ_4KB_SHIFT;
#[static_assert]
static _SZ_4KB_ASSERT : bool = SZ_4KB == 4 * 1024;

pub const SZ_64KB_SHIFT : usize = 16;
pub const SZ_64KB : usize = 1 << SZ_64KB_SHIFT;
#[static_assert]
static _SZ_64KB_ASSERT : bool = SZ_64KB == 64 * 1024;


pub const SZ_512MB_SHIFT : usize = 29;
pub const SZ_512MB : usize = 1 << SZ_512MB_SHIFT;
