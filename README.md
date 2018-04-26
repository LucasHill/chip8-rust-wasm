# CHIP-8 in Rust + Webassembly

## Overview
This is a fun demo of a chip-8 interpreter written in Rust, intended to be used in the browser using webassembly. A demo site is included to use the compiled web assembly. The WASM/JS memory layer is managed by wasm-bindgen which helps you write much more sane rust structs/methods which can be exported to be used in JavaScript.

## Prerequisites
* Node.JS + NPM
* Yarn
* Rust (nightly)
* wasm-bindgen-cli
* Firefox (Bugs in webpack prevent working in chrome)

## Build
* `./build_and_serve`

## References that helped me
* https://doc.rust-lang.org/stable/rust-by-example/
* http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
* http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/
* https://github.com/ColinEberhardt/wasm-rust-chip8
* https://github.com/starrhorne/chip8-rust/