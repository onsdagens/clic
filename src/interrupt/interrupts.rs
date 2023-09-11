#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Interrupts{
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

pub unsafe trait InterruptNumber{
    fn number(&self)->u16;
}

unsafe impl InterruptNumber for Interrupts{
    fn number(&self)->u16 {
        *self as u16
    }
}