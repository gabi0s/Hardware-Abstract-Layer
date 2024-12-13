#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Fonction de gestion des pannes
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Code spécifique à RISC-V
#[cfg(feature = "riscv")]
mod riscv {
    use super::*;

    // Exemple de registres génériques pour RISC-V (adresse à adapter selon votre matériel)
    const GPIO_OUT_REG: *mut u32 = 0x40000000 as *mut u32;  // Exemple d'adresse pour les registres GPIO
    const GPIO_ENABLE_REG: *mut u32 = 0x40001000 as *mut u32;  // Exemple d'adresse pour GPIO_ENABLE
    const GPIO_IN_REG: *const u32 = 0x40002000 as *const u32; // Exemple d'adresse pour GPIO_IN

    // Registres pour SPI (adresses à adapter selon votre matériel)
    const SPI_CTRL_REG: *mut u32 = 0x40003000 as *mut u32;  // Contrôle SPI
    const SPI_STATUS_REG: *const u32 = 0x40003004 as *const u32;  // Statut SPI
    const SPI_DATA_REG: *mut u32 = 0x40003008 as *mut u32;  // Registre de données SPI

    // Déclaration des registres I2C
    const I2C_CTRL_REG: *mut u32 = 0x40004000 as *mut u32;
    const I2C_STATUS_REG: *const u32 = 0x40004004 as *const u32;
    const I2C_DATA_REG: *mut u32 = 0x40004008 as *mut u32;

    // Fonctions pour configurer un GPIO en sortie et entrée
    fn set_gpio_output(gpio_num: u32) {
        unsafe {
            *GPIO_ENABLE_REG |= 1 << gpio_num;
        }
    }

    fn set_gpio_input(gpio_num: u32) {
        unsafe {
            *GPIO_ENABLE_REG &= !(1 << gpio_num);
        }
    }

    fn gpio_write(gpio_num: u32, value: bool) {
        unsafe {
            if value {
                *GPIO_OUT_REG |= 1 << gpio_num;
            } else {
                *GPIO_OUT_REG &= !(1 << gpio_num);
            }
        }
    }

    fn gpio_read(gpio_num: u32) -> bool {
        unsafe {
            (*GPIO_IN_REG & (1 << gpio_num)) != 0
        }
    }

    // Fonction d'initialisation du SPI
    fn spi_init() {
        unsafe {
            // Configurer SPI: par exemple, mode maître, fréquence de l'horloge, etc.
            *SPI_CTRL_REG = 0x01;  // Valeur fictive à adapter selon le matériel
        }
    }

    // Fonction pour envoyer un byte via SPI
    fn spi_send(data: u8) {
        unsafe {
            *SPI_DATA_REG = data as u32;  // Écriture dans le registre de données SPI
            // Attente de la fin de la transmission
            while *SPI_STATUS_REG & 0x01 == 0 {}  // Attente d'un statut de transmission terminé
        }
    }

    // Fonction pour recevoir un byte via SPI
    fn spi_receive() -> u8 {
        unsafe {
            // Attente que la donnée soit disponible
            while *SPI_STATUS_REG & 0x02 == 0 {}  // Attente d'un statut de réception
            (*SPI_DATA_REG & 0xFF) as u8  // Lire la donnée reçue
        }
    }

    // Initialisation I2C
    fn i2c_init() {
        unsafe {
            *I2C_CTRL_REG = 0x01;  // Activer le contrôleur I2C
        }
    }

    // Écriture d'un octet sur le bus I2C
    fn i2c_write_byte(addr: u8, data: u8) {
        unsafe {
            *I2C_DATA_REG = ((addr as u32) << 1) | 0;  // Adresse en écriture
            while *I2C_STATUS_REG & 0x01 == 0 {}  // Attente du périphérique prêt

            *I2C_DATA_REG = data as u32;  // Charger les données
            while *I2C_STATUS_REG & 0x02 == 0 {}  // Attente de la fin de l'envoi
        }
    }

    // Lecture d'un octet sur le bus I2C
    fn i2c_read_byte(addr: u8) -> u8 {
        unsafe {
            *I2C_DATA_REG = ((addr as u32) << 1) | 1;  // Adresse en lecture
            while *I2C_STATUS_REG & 0x01 == 0 {}  // Attente du périphérique prêt

            *I2C_DATA_REG as u8  // Retourner les données
        }
    }

    // Code principal pour RISC-V
    #[no_mangle]
    pub extern "C" fn main() -> ! {
        // Définir les numéros de GPIO pour SPI (exemple pour MOSI, MISO, SCK, CS)
        const GPIO_MOSI: u32 = 5;  // MOSI (Master Out Slave In)
        const GPIO_MISO: u32 = 6;  // MISO (Master In Slave Out)
        const GPIO_SCK: u32 = 7;   // SCK (Serial Clock)
        const GPIO_CS: u32 = 8;    // CS (Chip Select)

        // Configurer les GPIO en sortie (MOSI, SCK, CS) et entrée (MISO)
        set_gpio_output(GPIO_MOSI);
        set_gpio_output(GPIO_SCK);
        set_gpio_output(GPIO_CS);
        set_gpio_input(GPIO_MISO);

        // Initialisation du SPI
        spi_init();

        // Exemple d'envoi et de réception de données SPI
        loop {
            gpio_write(GPIO_CS, false);  // Sélectionner le périphérique SPI (mettre CS à LOW)

            // Envoi d'une donnée via SPI
            let data_to_send: u8 = 0xA5;
            spi_send(data_to_send);  // Envoi du byte

            // Réception d'une donnée via SPI
            let received_data = spi_receive();  // Réception du byte

            // Traitement de la donnée reçue (par exemple, afficher dans un registre ou autre)
            // Ici, nous avons juste une boucle infinie, mais vous pouvez ajouter votre logique

            gpio_write(GPIO_CS, true);  // Désélectionner le périphérique SPI (mettre CS à HIGH)
        }

        i2c_init();
        let addr = 0x3C;  // Adresse du périphérique
        i2c_write_byte(addr, 0x55);  // Envoyer un octet
        let received = i2c_read_byte(addr);  // Lire un octet
        loop {}
    }
}

#[cfg(feature = "atmega")]
mod atmega {
    use super::*;

    // Adresses des registres GPIO pour ATmega328P
    const PORTB: *mut u8 = 0x25 as *mut u8; // Registre PORTB (sortie)
    const DDRB: *mut u8 = 0x24 as *mut u8;  // Registre DDRB (direction)
    const PINB: *const u8 = 0x23 as *const u8; // Registre PINB (entrée)

    // Registres SPI pour ATmega328P
    const SPCR: *mut u8 = 0x2C as *mut u8;   // Registre de contrôle SPI
    const SPSR: *mut u8 = 0x2D as *mut u8;   // Registre de statut SPI
    const SPDR: *mut u8 = 0x2E as *mut u8;   // Registre de données SPI

    // Registres I2C pour ATmega328P
    const TWBR: *mut u8 = 0x20 as *mut u8;
    const TWSR: *mut u8 = 0x21 as *mut u8;
    const TWCR: *mut u8 = 0x56 as *mut u8;
    const TWDR: *mut u8 = 0x23 as *mut u8;

    // Fonctions pour configurer les GPIO
    fn set_gpio_output(pin: u8) {
        unsafe {
            *DDRB |= 1 << pin; // Définit le bit correspondant à "1" pour une sortie
        }
    }

    fn set_gpio_input(pin: u8) {
        unsafe {
            *DDRB &= !(1 << pin); // Définit le bit correspondant à "0" pour une entrée
        }
    }

    fn gpio_write(pin: u8, value: bool) {
        unsafe {
            if value {
                *PORTB |= 1 << pin;  // Met la broche à HIGH
            } else {
                *PORTB &= !(1 << pin); // Met la broche à LOW
            }
        }
    }

    fn gpio_read(pin: u8) -> bool {
        unsafe {
            (*PINB & (1 << pin)) != 0 // Retourne true si la broche est à HIGH
        }
    }

    // Initialisation SPI pour ATmega328P
    fn spi_init() {
        unsafe {
            *SPCR = (1 << 6) | (1 << 4) | (1 << 5); // Activer SPI, mode maître, fréquence Fclk/16
        }
    }

    // Envoi d'un octet via SPI
    fn spi_send(data: u8) {
        unsafe {
            *SPDR = data;  // Charger la donnée dans le registre SPI
            while *SPSR & (1 << 7) == 0 {} // Attendre la fin de la transmission (le bit SPIF à 1)
        }
    }

    // Réception d'un octet via SPI
    fn spi_receive() -> u8 {
        spi_send(0xFF);  // Envoie d'un octet fictif pour générer l'horloge SPI
        unsafe { *SPDR }  // Lire la donnée reçue
    }

    fn i2c_init() {
        unsafe {
            *TWBR = 32;
            *TWSR = 0x00;
            *TWCR = 1 << 6;  // Activer TWI
        }
    }

    fn i2c_start() {
        unsafe {
            *TWCR = (1 << 7) | (1 << 5) | (1 << 2);  // Condition START
            while (*TWCR & (1 << 7)) == 0 {}  // Attendre la fin
        }
    }

    fn i2c_write(data: u8) {
        unsafe {
            *TWDR = data;
            *TWCR = (1 << 7) | (1 << 2);
            while (*TWCR & (1 << 7)) == 0 {}
        }
    }

    fn i2c_read(ack: bool) -> u8 {
        unsafe {
            *TWCR = (1 << 7) | (1 << 2) | if ack { 1 << 6 } else { 0 };
            while (*TWCR & (1 << 7)) == 0 {}
            *TWDR
        }
    }

    // Fonction principale pour ATmega328P
    #[no_mangle]
    pub extern "C" fn main() -> ! {
        // Définir les broches utilisées pour SPI (MOSI, MISO, SCK, CS)
        const PIN_MOSI: u8 = 3;  // MOSI (Master Out Slave In)
        const PIN_MISO: u8 = 4;  // MISO (Master In Slave Out)
        const PIN_SCK: u8 = 5;   // SCK (Serial Clock)
        const PIN_CS: u8 = 2;    // CS (Chip Select)

        // Configurer les broches en sortie ou entrée
        set_gpio_output(PIN_MOSI);
        set_gpio_input(PIN_MISO);
        set_gpio_output(PIN_SCK);
        set_gpio_output(PIN_CS);

        // Initialisation du SPI
        spi_init();

        // Exemple d'envoi et de réception de données SPI
        loop {
            gpio_write(PIN_CS, false);  // Sélectionner le périphérique SPI (mettre CS à LOW)

            // Envoi d'un octet via SPI
            let data_to_send: u8 = 0xA5;
            spi_send(data_to_send);  // Envoi du byte

            // Réception d'un octet via SPI
            let received_data = spi_receive();  // Réception du byte

            // Traitement de la donnée reçue (par exemple, afficher dans un registre ou autre)
            // Vous pouvez ajouter ici du code pour traiter la donnée reçue

            gpio_write(PIN_CS, true);  // Désélectionner le périphérique SPI (mettre CS à HIGH)
        }

        i2c_init();
        i2c_start();
        i2c_write(0x3C << 1);  // Adresse périphérique en écriture
        i2c_write(0x55);  // Donnée
        loop {}

    }
}
