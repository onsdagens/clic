macro_rules! read_csr {
    ($csr_number:literal) => {
        /// Reads the CSR
        #[inline]
        unsafe fn _read() -> usize {
            match () {
                () => {
                    let r: usize;
                    core::arch::asm!(concat!("csrrs {0}, ", stringify!($csr_number), ", x0"), out(reg) r);
                    r
                }
            }
        }
    };
}
macro_rules! read_csr_as_usize {
    ($csr_number:literal) => {
        read_csr!($csr_number);

        /// Reads the CSR
        #[inline]
        pub fn read() -> usize {
            unsafe { _read() }
        }
    };
}
macro_rules! write_csr {
    ($csr_number:literal) => {
        /// Writes the CSR
        #[inline]
        #[allow(unused_variables)]
        unsafe fn _write(bits: usize) {
            match () {
                () => core::arch::asm!(concat!("csrrw x0, ", stringify!($csr_number), ", {0}"), in(reg) bits),
            }
        }
    };
}
macro_rules! write_csr_as_usize {
    ($csr_number:literal) => {
        write_csr!($csr_number);

        /// Writes the CSR
        #[inline]
        pub fn write(bits: usize) {
            unsafe { _write(bits) }
        }
    };
}