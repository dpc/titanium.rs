//! Convenient macro to read a CPU register

/// dsb instruction
#[macro_export]
macro_rules! dsb {
    () => {
        {
            unsafe {
                asm!("dsb");
            }
        }
    }
}

/// dmb instruction
#[macro_export]
macro_rules! dmb {
    () => {
        {
            unsafe {
                asm!("dmb");
            }
        }
    }
}

/// isb instruction
#[macro_export]
macro_rules! isb {
    () => {
        {
            unsafe {
                asm!("isb");
            }
        }
    }
}

#[macro_export]
macro_rules! def_reg_read {
    ( $reg:ident, $t:ty ) => {
            #[inline]
            pub fn read() -> $t {
                let mut val : $t;
                unsafe {
                    asm!(concat!("mrs $0, ", stringify!($reg))
                         : "=r"(val)
                        );
                }
                val
            }
    }
}
#[macro_export]
macro_rules! def_reg_write {
    ( $reg:ident, $t:ty ) => {
        #[inline]
        pub fn write(val : $t) {
            unsafe {
                asm!(concat!("msr ", stringify!($reg), ", $0" )
                     :
                     : "r"(val)
                    );
            }
        }
    }
}

#[macro_export]
macro_rules! def_reg {
    ($reg:ident, $t:ty, rw, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_read!($reg, $t);
            def_reg_write!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };
    ($reg:ident, $t:ty, ro, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_read!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };
    ($reg:ident, $t:ty, wo, $($field:ident($l:expr, $r:expr),)* ) => {
        #[allow(non_snake_case)]
        pub mod $reg {
            def_reg_write!($reg, $t);
            def_bitfields!($t, $($field($l, $r),)*);
        }
    };

}
