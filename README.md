# Chip8 Emulation

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/language-Rust-orange)

A CHIP-8 emulator written in Rust.  
**The Hello World of Emulation!**

---

## Table of Contents

- [About](#about)
- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)

---

## About

This repository contains a CHIP-8 emulator implemented in Rust. CHIP-8 is a simple, interpreted programming language, historically used for video game development on 1970s-80s microcomputers. This project is designed as an educational resource for learning about emulation, low-level system design, and Rust programming.

---

## Features

- Interprets CHIP-8 instructions and runs CHIP-8 ROMs.
- Written entirely in Rust for safety and performance.
- Modular, extensible codebase—ideal for learning and hacking.
- Command-line interface for loading and running ROMs.
- MIT Licensed and open for community contributions.

---

## Getting Started

### Prerequisites

- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)
- A CHIP-8 ROM file (`.ch8`)

### Clone the Repository

```sh
git clone https://github.com/Jayesh-Dev21/Chip8_emulation.git
cd Chip8_emulation
```

### Build

```sh
cargo build --release
```

### Run

```sh
cargo run --release -- <path-to-rom>
```

Replace `<path-to-rom>` with the path to your CHIP-8 ROM file. I have provided a wide collection of ROMs as well.

---

## Usage

1. Download or create a CHIP-8 ROM (for example, `UFO`).
2. Run the emulator from the command line, specifying your ROM file:

   ```sh
   cargo run --release -- roms/UFO
   ```

3. The emulator window will open and execute the CHIP-8 program.

---


## Project Structure

```
Chip8_emulation/
├── .gitignore                # Git ignore rules for the whole repo
├── LICENSE                   # Project license (MIT)
├── README.md                 # This file
├── game_catalog.txt          # Text file listing CHIP-8 games/ROMs
├── ROMS/                     # Directory for CHIP-8 ROM files (add your own here)
├── chip8_core/               # Core CHIP-8 emulator logic as a Rust library
│   ├── .gitignore
│   ├── Cargo.toml            # Rust crate manifest for the core
│   ├── Cargo.lock
│   └── src/                  # Source code for the core emulator
├── desktop/                  # Desktop (CLI/GUI) application using the core library
│   ├── .gitignore
│   ├── Cargo.toml            # Rust crate manifest for the desktop app
│   ├── Cargo.lock
│   └── src/                  # Source code for the desktop app
```

### Main Components

- **chip8_core/**: Contains the CHIP-8 emulation logic, implemented as a Rust library. This is where the CPU, memory, instruction decoder, etc. are found.
- **desktop/**: Contains the desktop application (CLI or GUI) that uses the core emulator library.
- **ROMS/**: You can put your CHIP-8 ROMs here to run them with the emulator.
- **game_catalog.txt**: Keeps track of available/tested games and ROMs.

---

This layout makes it easy to separate reusable core emulation code and the desktop front-end, supporting good modular Rust project practices.

## Contributing

Contributions are welcome! If you find bugs or have suggestions for improvements, please open an issue or submit a pull request.

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes.
4. Open a pull request describing your changes.

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- Rust and the open-source emulation community

---

> Made with ❤️ by [Jayesh-Dev21](https://github.com/Jayesh-Dev21)
