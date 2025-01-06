pub mod riscv {
    const GPIO_OUT_REG: *mut u32 = 0x40000000 as *mut u32;
    const GPIO_ENABLE_REG: *mut u32 = 0x40001000 as *mut u32;

    pub fn configure_gpio() {
        unsafe {
            *GPIO_ENABLE_REG |= 1 << 2;
        }
    }

    pub fn gpio_write(pin: u32, value: bool) {
        unsafe {
            if value {
                *GPIO_OUT_REG |= 1 << pin;
            } else {
                *GPIO_OUT_REG &= !(1 << pin);
            }
        }
    }
}

pub mod atmega {
    const PORTB: *mut u8 = 0x25 as *mut u8;
    const DDRB: *mut u8 = 0x24 as *mut u8;

    pub fn configure_gpio() {
        unsafe {
            *DDRB |= 1 << 1;
        }
    }

    pub fn gpio_write(pin: u8, value: bool) {
        unsafe {
            if value {
                *PORTB |= 1 << pin;
            } else {
                *PORTB &= !(1 << pin);
            }
        }
    }
}