Projet de **Gabriel Calmel** et **Caroline Bourdet** en **OCC1**

[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
Consider subdividing your project into separate modules.
You could abstract your function more, by putting your register adress out of the function.
Any operations here will be limited to the I/O registers of port B.

[CORRECTION SPI] (don't hesitate to remove this part)
You should implement the peripheral/slave mode as well (not only the controler/master mode).
You could organize your project into multiple file and folder (module file/folder for example).
For your RISC target, you should abstract the modification of register. You need to adapt your code to a specific target :
- If you don't want to choose a specific esp32, you have to make function that are generic enough, and that cover the differents types of esp32 (you have to adapt the multiple hardware yourself, not letting the user do it)
- You can choose a specific target, and design your HAL to it.

# Project HAL
## Description
Ce projet implémente une couche d'abstraction matérielle (HAL) pour interagir avec des périphériques tels que GPIO, SPI, I2C et UART sur deux architectures principales :
- **RISC-V (e.g., ESP32)**.
- **ATmega328P**.

Le projet est conçu pour fonctionner en mode `no_std` et utilise Rust avec des fonctionnalités spécifiques pour chaque architecture.



---

## Prérequis

### Installation des outils
1. **Rust Nightly** :
   Installez la version nightly de Rust et ajoutez les composants nécessaires :
   ```bash
   rustup install nightly
   rustup default nightly
   rustup component add rust-src --toolchain nightly
   ```

2. **Cibles pour les architectures** :
   - **Pour RISC-V** :
     ```bash
     rustup target add riscv32-unknown-none-elf
     ```
   - **Pour ATmega328P** :
     ```bash
     rustup target add avr-unknown-gnu-atmega328
     ```

3. **Linkers** :
   - Installez le linker GCC pour RISC-V :
     ```bash
     sudo apt install gcc-riscv64-unknown-elf
     ```
   - Installez `avrdude` pour flasher l'ATmega328P :
     ```bash
     sudo apt install avrdude
     ```

---

## Compilation

### Pour RISC-V (ESP32)
Utilisez la commande suivante pour compiler le projet pour une architecture RISC-V :
```bash
cargo +nightly build --release --target riscv32-unknown-none-elf --features riscv
```

### Pour ATmega328P
Utilisez cette commande pour compiler pour l'ATmega328P :
```bash
cargo +nightly build --release --target avr-unknown-gnu-atmega328 --features atmega
```

---

## Résolution (s'il y en a) des Problèmes

### 1. **Erreurs de Compatibilité des Versions**
Lors de l'installation des dépendances dans `Cargo.toml`, vous pourriez rencontrer des conflits. Essayez de :
- Vérifier que les versions de `riscv`, `riscv-rt` et `embedded-hal` sont compatibles.
- Modifier les versions des dépendances si nécessaire.

### 2. **Problèmes avec les cibles**
- Assurez-vous que les cibles RISC-V et ATmega328P sont bien ajoutées :
  ```bash
  rustup target list --installed
  ```
  Si elles ne sont pas présentes, utilisez :
  ```bash
  rustup target add <nom_de_la_cible>
  ```

### 3. **Erreurs de Linker**
Si le linker n'est pas trouvé, installez le linker approprié :
- **Pour RISC-V** : GCC RISC-V.
- **Pour ATmega328P** : `avrdude` ou autre outil adapté.

### 4. **Conflits entre les fichiers locaux et le dépôt distant**
Si vous ne pouvez pas pousser votre projet sur GitHub, utilisez :
```bash
git pull --rebase origin master
```
Ensuite :
```bash
git push origin master
```
---

## Fonctionnalités
- **GPIO** : Lecture et écriture sur les broches numériques.
- **SPI** : Modes maître et esclave pour la communication série.
- **I2C** : Envoi et réception de données.
- **UART** : Communication série bidirectionnelle.

### GPIO (General Purpose Input/Output)
Le module GPIO permet de configurer les broches en mode sortie et de contrôler leur état (haut ou bas).

#### Choix d’implémentation
- **RISC-V** : Les registres GPIO sont abstraits via des adresses mémoires fixes.
- **ATmega328P** : Les registres PORTB et DDRB ont été utilisés pour configurer et manipuler les broches.

#### Fonctionnalités principales
- Configuration des GPIO (sortie uniquement).
- Contrôle du niveau logique d'une broche spécifique.

#### Exemple d'utilisation
- **RISC-V** : 
  ```rust
  gpio::riscv::gpio_write(2, true); // Active la broche 2
  ```


### SPI (Serial Peripheral Interface)
Le module SPI prend en charge les modes Maître et Esclave pour permettre la communication entre périphériques via un bus série.

#### Choix d’implémentation
- **RISC-V** : L'initialisation et la gestion SPI sont abstraites à l'aide de registres mémoires spécifiques.
- **ATmega328P** : Les registres SPCR (SPI Control Register) ont été utilisés pour configurer le périphérique SPI.

#### Fonctionnalités principales
- Initialisation du SPI en mode Maître et Esclave.
- Envoi et réception de données.

#### Exemple d'utilisation
- **RISC-V** : 
```rust
spi::riscv::spi_send(0xA5); // Envoie une donnée en mode Maître 
```

- **ATmega328P** : 
```rust
spi::atmega::spi_init(); // Initialise le module SPI
```


### UART (Universal Asynchronous Receiver-Transmitter)
Le module UART offre une communication série bidirectionnelle, utile pour le débogage ou l'interaction avec des périphériques externes.

#### Choix d’implémentation
- **RISC-V** : Gestion via les registres de contrôle et de données UART.
- **ATmega328P** : Configuration des registres UBRR0 pour définir le baud rate.

#### Fonctionnalités principales
- Initialisation du module UART.
- Envoi et réception de données.

#### Exemple d'utilisation
- **RISC-V** : 
```rust
uart::riscv::uart_send(b"Hello, UART!\n");
```
- **ATmega328P** : 
```rust
uart::atmega::uart_receive();
```


### I2C (Inter-Integrated Circuit)
Le module I2C permet la communication entre un Maître et un ou plusieurs Esclaves via un bus partagé.

#### Choix d’implémentation
- **RISC-V** : Abstraction via un registre de contrôle spécifique.
- **ATmega328P** : Configuration des registres TWBR pour contrôler la vitesse du bus I2C.

#### Fonctionnalités principales
- Initialisation du bus I2C.
- Envoi et réception de données.

#### Exemple d'utilisation
- **RISC-V** : 
```rust
i2c::riscv::i2c_init();
```
- **ATmega328P** : 
```rust
i2c::atmega::i2c_write(0x55); // Envoie une donnée à un Esclave
```

---

## Etape SPI 
Lors de laquelle on ajoute la fonctionnalité SPI en utilisant la bibliothèque HAL de riscv pour l'intégrer sur l'esp32.

1. **Initialisation du SPI** :
   Nous avons créé une instance SPI en spécifiant le périphérique SPI à utiliser (par exemple, SPI2) et en définissant les broches MISO et MOSI.

2. **Envoi et réception de données** :
   Nous avons implémenté des fonctions pour envoyer et recevoir des données via SPI. L'envoi est réalisé en utilisant `spi.write(&[data])` et la réception se fait avec `spi.read(&mut buffer)`.

### SPI pour l'ATmega328P
Pour l'ATmega328P, nous avons directement manipulé les registres SPI disponibles dans le microcontrôleur. Voici comment nous avons procédé :

1. **Définition des registres SPI** :
   Nous avons défini les registres nécessaires pour contrôler le SPI, tels que le registre de contrôle (SPCR), le registre d'état (SPSR) et le registre de données (SPDR).

2. **Initialisation du SPI** :
   Nous avons créé une fonction d'initialisation qui configure les registres appropriés pour activer le mode maître et définir la fréquence d'horloge.

3. **Fonctions d'envoi et de réception** :
   Nous avons également créé des fonctions pour envoyer et recevoir des données via SPI (`spi_send` et `spi_receive`).

### Conclusion
L'ajout du support SPI pour nos deux cibles a amélioré les capacités de communication de notre projet, simplifiant l'interaction avec différents périphériques externes. Notre système peut désormais gérer efficacement les communications série, aussi bien sur l'ESP32 que sur l'ATmega328P.


---

## Problèmes rencontrés

1. **Compatibilité des versions** : Ajouter des dépendances dans le fichier Cargo.toml a généré des conflits entre certaines bibliothèques (comme esp-backtrace et esp-println). Cela nous a obligés à ajuster les versions pour garantir qu'elles fonctionnent ensemble.

2. **Configuration des cibles** : Configurer les cibles pour l'ESP32 et l'ATmega328P a demandé une attention particulière. Nous avons dû vérifier que les toolchains appropriées étaient bien installées et que les cibles étaient correctement intégrées à notre environnement Rust. De plus, en ayant changé d'ordinateur, la moitié de l'équipe n'a pas réussi à correctement intsaller les 2 cibles donc nous n'avons pas pu vérifier correctement que notre code était fonctionnel.

3. **Mode no_std** : Travailler en mode no_std a posé des défis supplémentaires, comme l'absence de certaines fonctionnalités de la bibliothèque standard. Nous avons dû utiliser des bibliothèques adaptées aux systèmes embarqués et bien comprendre les limites de ce type de développement sans allocation dynamique.

4. **Point d'entrée** : Malgré le "#[no_mangle]" devant les fonctions main, cette erreur peut persister : "LINK : fatal error LNK1561: le point d'entrée doit être défini".








