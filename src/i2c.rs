pub mod riscv {
    const I2C_CTRL_REG: *mut u32 = 0x40004000 as *mut u32;

    pub fn i2c_init() {
        unsafe {
            *I2C_CTRL_REG = 0x01;
        }
    }
}

pub mod atmega {
    const TWBR: *mut u8 = 0x20 as *mut u8;

    pub fn i2c_init() {
        unsafe {
            *TWBR = 32;
        }
    }
}