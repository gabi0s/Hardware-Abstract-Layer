pub mod riscv {
    const UART_CTRL_REG: *mut u32 = 0x40005000 as *mut u32;
    const UART_DATA_REG: *mut u32 = 0x40005004 as *mut u32;

    pub fn uart_init() {
        unsafe {
            *UART_CTRL_REG = 0x01;
        }
    }

    pub fn uart_send(data: &[u8]) {
        for &byte in data {
            unsafe {
                *UART_DATA_REG = byte as u32;
            }
        }
    }

    pub fn uart_receive() -> Option<u8> {
        unsafe {
            Some(*UART_DATA_REG as u8)
        }
    }
}

pub mod atmega {
    const UBRR0H: *mut u8 = 0xC5 as *mut u8;
    const UBRR0L: *mut u8 = 0xC4 as *mut u8;
    const UCSR0B: *mut u8 = 0xC1 as *mut u8;
    const UDR0: *mut u8 = 0xC6 as *mut u8;

    pub fn uart_init() {
        unsafe {
            *UBRR0H = 0;
            *UBRR0L = 103;
            *UCSR0B = (1 << 3) | (1 << 4);
        }
    }

    pub fn uart_send(data: &[u8]) {
        for &byte in data {
            unsafe {
                *UDR0 = byte;
            }
        }
    }

    pub fn uart_receive() -> Option<u8> {
        unsafe {
            Some(*UDR0)
        }
    }
}
