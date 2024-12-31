Projet de Gabriel Calmel et Caroline Bourdet 0CC1

[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
Consider subdividing your project into separate modules.
You could abstract your function more, by putting your register adress out of the function.
Any operations here will be limited to the I/O registers of port B.

# Project HAL

## Etape 1

## Etape 2
On a ajouté une nouvelle cible dans notre environnement Rust pour pouvoir complier notre projet.

1. **Installation de la toolchain nécessaire** :
   Nous avons installé la version nightly de Rust avec le composant `rust-src` pour permettre la compilation en mode no_std :
   ```bash
   rustup toolchain install nightly
   rustup component add rust-src --toolchain nightly
   ```

2. **Ajout de la cible ESP32** :
   Nous avons ajouté la cible pour l'ESP32 en utilisant la commande suivante :
   ```bash
   rustup target add riscv64gc-unknown-none-elf
   ```

3. **Configuration du projet** :
   Dans notre fichier `Cargo.toml`, nous avons défini des features pour activer le code spécifique à l'ESP32. Cela nous permet de compiler le projet pour différentes cibles (ATmega328P et ESP32) en fonction des besoins.

4. **Compilation du projet** :
   Pour compiler le projet pour l'ESP32, nous utilisons la commande suivante :
   ```bash
   cargo +nightly build --target riscv64gc-unknown-none-elf --release --features riscv
   ```

## Etape 3 
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

## Prérequis
Assurez-vous d'avoir installé la version nightly de Rust ainsi que le composant rust-src :
```bash
rustup component add rust-src --toolchain nightly-x86_64-pc-windows-msvc
```

## Commande de compilation
Pour compiler le projet, utilisez la commande suivante pour l'ATmega328P:
```bash
cargo +nightly build --target avr-unknown-gnu-atmega328 --release --features atmega
```
ou pour l'ESP32 :
```bash
cargo +nightly build -Z build-std=core,alloc --target riscv64gc-unknown-none-elf --features riscv
```

## Problèmes rencontrés

1. **Compatibilité des versions** : Ajouter des dépendances dans le fichier Cargo.toml a généré des conflits entre certaines bibliothèques (comme esp-backtrace et esp-println). Cela nous a obligés à ajuster les versions pour garantir qu'elles fonctionnent ensemble.

2. **Configuration des cibles** : Configurer les cibles pour l'ESP32 et l'ATmega328P a demandé une attention particulière. Nous avons dû vérifier que les toolchains appropriées étaient bien installées et que les cibles étaient correctement intégrées à notre environnement Rust.

3. **Mode no_std** : Travailler en mode no_std a posé des défis supplémentaires, comme l'absence de certaines fonctionnalités de la bibliothèque standard. Nous avons dû utiliser des bibliothèques adaptées aux systèmes embarqués et bien comprendre les limites de ce type de développement sans allocation dynamique.







[CORRECTION SPI] (don't hesitate to remove this part)
You should implement the peripheral/slave mode as well (not only the controler/master mode).
You could organize your project into multiple file and folder (module file/folder for example).
For your RISC target, you should abstract the modification of register. You need to adapt your code to a specific target :
- If you don't want to choose a specific esp32, you have to make function that are generic enough, and that cover the differents types of esp32 (you have to adapt the multiple hardware yourself, not letting the user do it)
- You can choose a specific target, and design your HAL to it.