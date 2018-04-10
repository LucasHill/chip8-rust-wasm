import { Cartridge, CPU } from './chip8_rust_wasm';

const CANVAS_WIDTH = 64;
const CANVAS_HEIGHT = 32;

const mainCtx = initializeCanvas(CANVAS_WIDTH, CANVAS_HEIGHT);

(async function() {
  const cpu = await intializeEmulatorForGame('PONG');

  window.setInterval(execute_cycle.bind(this, cpu), 2);
})();

function execute_cycle(emulator) {
  const result = emulator.tick();
  console.log('tick');
  const displayState = result.get_display_state();
  updateCanvas(displayState, mainCtx, CANVAS_WIDTH, CANVAS_HEIGHT);
}


function initializeCanvas(width, height) {
  const canvas = document.getElementById("game");
  const ctx = canvas.getContext("2d");
  ctx.fillStyle = "black";
  ctx.fillRect(0, 0, width, height);

  return ctx;
}

function updateCanvas(displayState, ctx, width, height) {
  const imageData = ctx.createImageData(width, height);
  for (let i = 0; i < displayState.length; i++) {
    imageData.data[i * 4] = displayState[i] === 1 ? 0x33 : 0;
    imageData.data[i * 4 + 1] = displayState[i] === 1 ? 0xff : 0;
    imageData.data[i * 4 + 2] = displayState[i] === 1 ? 0x66 : 0;
    imageData.data[i * 4 + 3] = 255;
  }
  ctx.putImageData(imageData, 0, 0);
}

async function intializeEmulatorForGame(name) {
  const response = await window.fetch(`roms/${name}`);
  const game = await response.arrayBuffer();
  console.log(game);
  const cartridge = Cartridge.new(new Uint8Array(game));

  return CPU.new(cartridge)
}

