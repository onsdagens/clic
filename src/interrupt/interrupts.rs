use core::arch::asm;

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Interrupt{
    Interrupt0 = 0,
    Interrupt1 = 1,
    Interrupt2 = 2,
    Interrupt3 = 3,
    Interrupt4 = 4,
    Interrupt5 = 5,
    Interrupt6 = 6,
    Interrupt7 = 7,
    Interrupt8 = 8,
    Interrupt9 = 9,
    Interrupt10 = 10,
    Interrupt11 = 11,
    Interrupt12 = 12,
    Interrupt13 = 13,
    Interrupt14 = 14,
    Interrupt15 = 15,
    Interrupt16 = 16,
    Interrupt17 = 17,
    Interrupt18 = 18,
    Interrupt19 = 19,
    Interrupt20 = 20,
    Interrupt21 = 21,
    Interrupt22 = 22,
}

pub struct Interrupt0;
pub struct Interrupt1;
pub struct Interrupt2;
pub struct Interrupt3;
pub struct Interrupt4;
pub struct Interrupt5;
pub struct Interrupt6;
pub struct Interrupt7;
pub struct Interrupt8;
pub struct Interrupt9;

pub unsafe trait Pend{
    unsafe fn pend_int();
}

unsafe impl Pend for Interrupt0 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB00, 1");
    }
}
unsafe impl Pend for Interrupt1 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB01, 1");
    }
}
unsafe impl Pend for Interrupt2 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB02, 1");
    }
}
unsafe impl Pend for Interrupt3 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB03, 1");
    }
}
unsafe impl Pend for Interrupt4 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB04, 1");
    }
}
unsafe impl Pend for Interrupt5 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB05, 1");
    }
}
unsafe impl Pend for Interrupt6 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB06, 1");
    }
}
unsafe impl Pend for Interrupt7 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB07, 1");
    }
}
unsafe impl Pend for Interrupt8 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB08, 1");
    }
}
unsafe impl Pend for Interrupt9 {
    #[inline(always)]
    unsafe fn pend_int() {
        asm!("csrrsi zero, 0xB09, 1");
    }
}

pub unsafe trait InterruptNumber{
    fn number(&self)->u16;
}

unsafe impl InterruptNumber for Interrupt{
    fn number(&self)->u16 {
        *self as u16
    }
}
