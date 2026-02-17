# Chip8 Emulation

[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
![Rust](https://img.shields.io/badge/language-Rust-orange)
![WebAssembly](https://img.shields.io/badge/wasm-supported-purple)

A CHIP-8 emulator written in Rust with WebAssembly support.  
**The Hello World of Emulation!**

<img width="1149" height="870" alt="ss" src="https://github.com/user-attachments/assets/9c4fb95e-ab8a-49cb-b548-3997c3fb8953" />

---

## About

CHIP-8 is a simple, interpreted programming language from the 1970s used for video games on early microcomputers. This project implements a complete CHIP-8 emulator in Rust, featuring both a native desktop version (SDL2) and a web version (WebAssembly).

---

## Features

- ✅ Full CHIP-8 instruction set implementation
- 🦀 Written in Rust for safety and performance
- 🖥️ **Desktop version** with SDL2 graphics and audio
- 🌐 **Web version** with WebAssembly (runs in browser)
- 🎮 Includes 20+ classic CHIP-8 games
- 🎵 Sound timer support with beep audio
- ⌨️ Keyboard controls for both desktop and web
- 📦 Modular architecture with reusable core library

---

## Getting Started

### Desktop Version

**Prerequisites:**
- [Rust toolchain](https://www.rust-lang.org/tools/install) (stable)
- SDL2 development libraries

**Build and Run:**
```sh
git clone https://github.com/Jayesh-Dev21/Chip8_emulation.git
cd Chip8_emulation/rust/desktop
cargo run --release
```

### Web Version

**Prerequisites:**
- Rust toolchain with `wasm32-unknown-unknown` target
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

**Build:**
```sh
cd rust/wasm
wasm-pack build --target web --out-dir ../../web/pkg --release
```

**Serve locally:**
```sh
cd web
python3 -m http.server 8000
# Open http://localhost:8000
```

---

## Controls

**Keyboard Mapping:**
```
CHIP-8 Keypad:    Your Keyboard:
1 2 3 C           1 2 3 4
4 5 6 D    =>     Q W E R
7 8 9 E           A S D F
A 0 B F           Z X C V
```


[showcase.webm](https://github.com/user-attachments/assets/9ac8e2f4-c963-40d1-9ca8-b1c11ec0f3c7)

---

## Project Structure

```
Chip8_emulation/
├── rust/
│   ├── chip8_core/          # Core emulator logic (library)
│   ├── desktop/             # SDL2 desktop application
│   └── wasm/                # WebAssembly wrapper
├── web/                     # Web UI (HTML/CSS/JS)
│   ├── pkg/                 # Generated wasm files
│   └── c8games/             # ROM files
├── ROMS/                    # Additional ROM collection
└── README.md
```

---

## Technical Details

- **Core Library**: Platform-agnostic CHIP-8 emulator
- **Desktop**: Uses SDL2 for graphics, input, and audio
- **Web**: Compiled to WebAssembly, runs in modern browsers
- **Audio**: Native rodio (desktop) / WebAudio API (web)
- **Random**: Deterministic fallback for wasm builds

---

## Contributing

Contributions welcome! Feel free to open issues or submit pull requests.

---

## License

MIT License - see [LICENSE](LICENSE) file for details.

---

## Acknowledgements

- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [Cowgod's CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- Rust and WebAssembly communities

---

> Made with ❤️ by [Jayesh-Dev21](https://github.com/Jayesh-Dev21)
