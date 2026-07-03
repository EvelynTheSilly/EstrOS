#![allow(dead_code)]
use core::hint::unlikely;

use crate::{rng::implementations::xorshift::XORShiftRng, syncronisation::GlobalSharedLock};

mod implementations;

pub trait Rng {
    fn rand_u64(&mut self) -> u64;
    fn rand_i64(&mut self) -> i64 {
        u64::cast_signed(self.rand_u64())
    }
    fn rand_u32(&mut self) -> u32 {
        u32::from_ne_bytes(self.rand_u64().to_ne_bytes()[..4].try_into().expect("
            SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
        "))
    }
    fn rand_i32(&mut self) -> i32 {
        u32::cast_signed(self.rand_u32())
    }
    fn rand_u16(&mut self) -> u16 {
        u16::from_ne_bytes(self.rand_u32().to_ne_bytes()[..2].try_into().expect("
            SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
        "))
    }
    fn rand_i16(&mut self) -> i16 {
        u16::cast_signed(self.rand_u16())
    }
    fn rand_u8(&mut self) -> u8 {
        u8::from_ne_bytes(self.rand_u16().to_ne_bytes()[..1].try_into().expect("
                SAFETY: shouldnt panic because the bytes are clamped to the size of the appropriate type known at compile time, shouldnt link to the panic handler in optimised code
            "))
    }
    fn rand_i8(&mut self) -> i8 {
        u8::cast_signed(self.rand_u8())
    }
    fn rand_u64_not_by(&mut self, fun: impl Fn(u64) -> bool) -> u64 {
        let mut ret = self.rand_u64();
        while unlikely(fun(ret)) {
            ret = self.rand_u64();
        }
        ret
    }
}

pub static RNG: GlobalSharedLock<XORShiftRng> = GlobalSharedLock::new(XORShiftRng::new(69420));
