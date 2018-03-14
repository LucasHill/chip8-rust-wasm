import { Cartridge, concat } from './chip8_rust_wasm';


const cartridge = Cartridge.new(new Uint8Array([69, 32]));
const resp = cartridge.get_output();
console.log(resp);