#[macro_export]
macro_rules! syscall {
    ($nr:expr $(, $arg:expr )* $(,)?) => {{
        let ret: usize;
        unsafe {
            $crate::syscall!(@build $nr, (lateout("x0") ret, options(nostack)) 0 $(, $arg )*)
        }
        ret
    }};

    (@build $nr:expr, ($($tail:tt)*) $idx:tt) => {
        core::arch::asm!(
            "svc {nr}",
            nr = const $nr,
            $($tail)*
        );
    };
    (@build $nr:expr, ($($tail:tt)*) $idx:tt,) => {
        core::arch::asm!(
            "svc {nr}",
            nr = const $nr,
            $($tail)*
        );
    };
    (@build $nr:expr, ($($tail:tt)*) 0, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x0") $a, $($tail)*) 1 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 1, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x1") $a, $($tail)*) 2 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 2, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x2") $a, $($tail)*) 3 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 3, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x3") $a, $($tail)*) 4 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 4, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x4") $a, $($tail)*) 5 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 5, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x5") $a, $($tail)*) 6 $(, $rest)*)
    };
    (@build $nr:expr, ($($tail:tt)*) 6, $a:expr $(, $rest:expr)*) => {
        $crate::syscall!(@build $nr, (in("x6") $a, $($tail)*) 7 $(, $rest)*)
    };
}
