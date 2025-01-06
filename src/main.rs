#![no_std]
#![no_main]

mod gpio;
mod spi;
mod i2c;
mod uart;

use core::panic::PanicInfo;

// Fonction de gestion des pannes
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cfg(feature = "riscv")]
mod riscv {
    use super::*;

    #[no_mangle]
    pub extern "C" fn main() -> ! {
        gpio::riscv::configure_gpio();
        spi::riscv::spi_init_master();
        spi::riscv::spi_init_slave();
        i2c::riscv::i2c_init();
        uart::riscv::uart_init();

        loop {
            uart::riscv::uart_send(b"Hello, UART!\n");
            if let Some(byte) = uart::riscv::uart_receive() {
                if byte == b'1' {
                    gpio::riscv::gpio_write(2, true);
                } else {
                    gpio::riscv::gpio_write(2, false);
                }
            }

            // SPI Slave example
            if let Some(data) = spi::riscv::spi_receive() {
                spi::riscv::spi_send(data + 1); // Echo modified data back
            }

            if let Some(byte) = uart::riscv::uart_receive() {
                gpio::riscv::gpio_write(2, byte == b'1');
            }
        }
    }
}

#[cfg(feature = "atmega")]
mod atmega {
    use super::*;

    #[no_mangle]
    pub extern "C" fn main() -> ! {
        gpio::atmega::configure_gpio();
        spi::atmega::spi_init_master();
        spi::atmega::spi_init_slave();
        i2c::atmega::i2c_init();
        uart::atmega::uart_init();

        loop {
            // Example 1: Toggle GPIO based on UART input
            uart::atmega::uart_send(b"Send '1' to turn LED on, '0' to turn it off.\n");
            if let Some(byte) = uart::atmega::uart_receive() {
                if byte == b'1' {
                    gpio::atmega::gpio_write(1, true); // Turn on LED
                    uart::atmega::uart_send(b"LED is ON\n");
                } else if byte == b'0' {
                    gpio::atmega::gpio_write(1, false); // Turn off LED
                    uart::atmega::uart_send(b"LED is OFF\n");
                }
            }

            // Example 2: SPI communication (Slave Mode)
            if let Some(received_data) = spi::atmega::spi_receive() {
                // Respond with modified data
                let response_data = received_data.wrapping_add(1);
                spi::atmega::spi_send(response_data);
            }

            // Example 3: I2C communication (Master Mode)
            i2c::atmega::i2c_start();
            i2c::atmega::i2c_write(0x3C << 1); // Address of the device
            i2c::atmega::i2c_write(0x55);      // Send data
            i2c::atmega::i2c_stop();

            // Delay (if necessary)
            for _ in 0..1_000_000 {
                core::hint::spin_loop();
            }
        }
    }
}