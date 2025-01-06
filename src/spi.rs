pub mod riscv {
    const SPI_CTRL_REG: *mut u32 = 0x40003000 as *mut u32;

    pub fn spi_init() {
        unsafe {
            *SPI_CTRL_REG = 0x01;
        }
    }
}

pub mod atmega {
    const SPCR: *mut u8 = 0x2C as *mut u8;

    pub fn spi_init() {
        unsafe {
            *SPCR = (1 << 6) | (1 << 4);
        }
    }
}