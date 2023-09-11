use core::marker::PhantomData;
use critical_section;

#[allow(non_snake_case, dead_code)]
pub struct Peripherals{
    pub CLIC:CLIC
}

static mut TAKEN: bool = false;

/// Nested Vector Interrupt Controller
#[allow(clippy::upper_case_acronyms)]
pub struct CLIC {
    _marker: PhantomData<*const ()>,
}


impl Peripherals {
    /// Returns all the core peripherals *once*
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { TAKEN } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }

    /// Unchecked version of `Peripherals::take`
    #[inline]
    pub unsafe fn steal() -> Self {
        TAKEN = true;

        Peripherals {
            CLIC: CLIC {
                _marker: PhantomData,
            },
        }
    }
}