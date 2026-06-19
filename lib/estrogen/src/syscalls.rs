#[macro_export]
macro_rules! syscall {
    ($nr:expr $(, $arg:expr )* $(,)?) => {{
        let ret: usize;
        unsafe {
            core::arch::asm!(
                "svc #0",
                in("x0") $nr,
                arm64_syscall!(@args 0 $(, $arg )*),
                lateout("x0") ret,
                options(nostack),
            );
        }
        ret
    }};

    (@args $idx:tt,) => {};
    (@args 0, $a:expr $(, $rest:expr)*) => {
        in("x1") $a,
        arm64_syscall!(@args 1 $(, $rest)*)
    };
    (@args 1, $a:expr $(, $rest:expr)*) => {
        in("x2") $a,
        arm64_syscall!(@args 2 $(, $rest)*)
    };
    (@args 2, $a:expr $(, $rest:expr)*) => {
        in("x3") $a,
        arm64_syscall!(@args 3 $(, $rest)*)
    };
    (@args 3, $a:expr $(, $rest:expr)*) => {
        in("x4") $a,
        arm64_syscall!(@args 4 $(, $rest)*)
    };
    (@args 4, $a:expr $(, $rest:expr)*) => {
        in("x5") $a,
        arm64_syscall!(@args 5 $(, $rest)*)
    };
    (@args 5, $a:expr $(, $rest:expr)*) => {
        in("x6") $a,
        arm64_syscall!(@args 6 $(, $rest)*)
    };
    (@args 6, $a:expr $(, $rest:expr)*) => {
        in("x7") $a,
    };
}
