#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Interrupts{
    Interrupt0 = 0,
    Interrupt1 = 1,
    Interrupt2 = 2,
}

pub unsafe trait InterruptNumber{
    fn number(&self)->u16;
}

unsafe impl InterruptNumber for Interrupts{
    fn number(&self)->u16 {
        *self as u16
    }
}