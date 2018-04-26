import('./chip8.js');

/*
*
* WASM External functions
*
*/
window.generateRandomU8 = function generateRandomU8() {
  return Math.floor(Math.random() * Math.floor(255));
}