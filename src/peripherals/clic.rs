use core::ops;
use volatile_register::RW;
use crate::interrupt::InterruptNumber;
use crate::peripherals::CLIC;

#[repr(C)]
pub struct RegisterBlock{
    pub interrupts:[[RW<u8>;4]; 4096]
}

unsafe impl Send for CLIC {}

impl CLIC {
    /// Pointer to the register block
    pub const PTR: *const RegisterBlock = 0x0000_1000 as *const _; //this should be some static pointing to base of CLIC + 0x1000, maybe have user use PROVIDES
}

impl ops::Deref for CLIC {
    type Target = RegisterBlock;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}

impl CLIC {
    /// Disables `interrupt`
    #[inline]
    pub fn mask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].write(0) }
    }

    /// Enables interrupt
    #[inline]
    pub unsafe fn unmask<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].write(1) }
    }

    /// Returns the CLIC priority of interrupt
    #[inline]
    pub fn get_priority<I>(interrupt: I) -> u8
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        // NOTE(unsafe) atomic read with no side effects
        unsafe { (*Self::PTR).interrupts[nr as usize][3].read() }
    }

    /// Checks if interrupt is enabled
    #[inline]
    pub fn is_enabled<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][1].read()!=0 }
    }

    /// Checks if interrupt is pending
    #[inline]
    pub fn is_pending<I>(interrupt: I) -> bool
    where
        I: InterruptNumber,
    { 
        let nr = interrupt.number();

        unsafe { (*Self::PTR).interrupts[nr as usize][0].read()!=0 }
    }

    /// Forces interrupt into pending state
    #[inline]
    pub fn pend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][0].write(1) }
    }

    /// Sets the priority of interrupt to prio
    #[inline]
    pub unsafe fn set_priority<I>(&mut self, interrupt: I, prio: u8)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][3].write(prio) }
    }

    /// Clears interrupt's pending state
    #[inline]
    pub fn unpend<I>(interrupt: I)
    where
        I: InterruptNumber,
    {
        let nr = interrupt.number();
        unsafe { (*Self::PTR).interrupts[nr as usize][0].write(0) }
    }
}
