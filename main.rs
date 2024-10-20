#![feature(asm_experimental_arch)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

unsafe fn set_pin_mode(pin: u8, mode: u8) {
    let ddrb = 0x24 as *mut u8;
    if mode == 1 {
        *ddrb |= 1 << pin; // Configurer la broche comme sortie
    } else {
        *ddrb &= !(1 << pin); // Configurer la broche comme entrée
    }
}

unsafe fn write_pin(pin: u8, value: u8) {
    let portb = 0x25 as *mut u8;
    if value == 1 {
        *portb |= 1 << pin;  // Mettre la broche à l'état haut
    } else {
        *portb &= !(1 << pin); // Mettre la broche à l'état bas
    }
}

unsafe fn read_pin(pin: u8) -> u8 {
    let pinb = 0x23 as *mut u8;
    (*pinb & (1 << pin)) >> pin
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        // Configuration des broches numériques, par exemple :
        // Configurer PB5 (broche 13 de l'Arduino) comme sortie
        set_pin_mode(5, 1);

        // Exemple d'écriture sur PB5 : allumer la broche
        write_pin(5, 1);

        // Lecture de l'état de PB5
        let _etat_pb5 = read_pin(5);

        // Boucle infinie pour éviter que le programme se termine
        loop {}
    }
}

// Gestionnaire de panic requis par le compilateur
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
