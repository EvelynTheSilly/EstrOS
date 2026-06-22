use core::fmt::Write;

pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::syscall!(1, s.as_ptr(), s.len());
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::syscall!(1, "\n".as_ptr(), "\n".len());
    };
    ($($arg:tt)*) => {{
        use ::core::fmt::Write;
        let mut w = $crate::io::Writer;
        let _ = ::core::write!(&mut w, $($arg)*);
        $crate::syscall!(1, "\n".as_ptr(), "\n".len());
    }};
}
