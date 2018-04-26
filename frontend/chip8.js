import { Cartridge, CPU } from './chip8_rust_wasm';

const CANVAS_WIDTH = 64;
const CANVAS_HEIGHT = 32;

const mainCtx = initializeCanvas(CANVAS_WIDTH, CANVAS_HEIGHT);

(async function() {
  const cpu = await intializeEmulatorForGame('PONG2');
  setupXboxController(cpu);
  setUpKeyboardListeners(cpu);
  window.setInterval(execute_cycle.bind(this, cpu), 2);
})();

function execute_cycle(emulator) {
  const result = emulator.tick();
  // console.log('tick');
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

function setUpKeyboardListeners(cpu) {
  document.addEventListener('keydown', event => {
    const key = event.key.toLowerCase();
    cpu.gamepad_down(key);
  });

  document.addEventListener('keyup', event => {
    const key = event.key;
    cpu.gamepad_up(key);
  });
}

function setupXboxController(cpu) {
  const gamepads = [];
  window.addEventListener("gamepadconnected", function(e) {
    console.log("Gamepad connected at index %d: %s. %d buttons, %d axes.",
      e.gamepad.index, e.gamepad.id,
      e.gamepad.buttons.length, e.gamepad.axes.length);

    gamepads.push(e.gamepad);
  });

  window.addEventListener("gamepaddisconnected", function(e) {
    console.log("Gamepad disconnected from index %d: %s",
    e.gamepad.index, e.gamepad.id);

    const index = gamepads.findIndex(gamepad => gamepad.id === e.gamepad.id);
    gamepads.slice(index, 1);
  });

  function checkGamepads() {
    if(gamepads.length > 1) {
      const up1 = gamepads[0].buttons[0]
      if(up1.pressed) {
        cpu.gamepad_down('2');
      } else {
        cpu.gamepad_up('2');
      }

      const down1 = gamepads[0].buttons[1]
      if(down1.pressed) {
        cpu.gamepad_down('q');
      } else {
        cpu.gamepad_up('q');
      }

      const up2 = gamepads[1].buttons[0]
      if(up2.pressed) {
        cpu.gamepad_down('z');
      } else {
        cpu.gamepad_up('z');
      }

      const down2 = gamepads[1].buttons[1]
      if(down2.pressed) {
        cpu.gamepad_down('x');
      } else {
        cpu.gamepad_up('x');
      }
    }

  }

  setInterval(checkGamepads, 200);
}
