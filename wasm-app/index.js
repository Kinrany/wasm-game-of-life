import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const UNIVERSE_WIDTH = 64;
const UNIVERSE_HEIGHT = UNIVERSE_WIDTH;
const UNIVERSE_DENSITY = 0.5;

const CELL_SIZE = 10; // px
const BORDER_SIZE = 1; // px
const CELL_OFFSET = CELL_SIZE + BORDER_SIZE; // px

const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, UNIVERSE_DENSITY);
const width = universe.width();
const height = universe.height();

/** @type {HTMLCanvasElement} */
const canvas = document.getElementById('game-of-life-canvas');
canvas.width = CELL_OFFSET * width + BORDER_SIZE;
canvas.height = CELL_OFFSET * height + BORDER_SIZE;

const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.clearRect(0, 0, canvas.width, canvas.height);

  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  ctx.lineWidth = BORDER_SIZE;

  // Vertical lines
  for (let i = 0; i <= width; ++i) {
    // i-th line is between cells i-1 and i
    const x = i * CELL_OFFSET + BORDER_SIZE / 2;
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
  }

  // Horizontal lines
  for (let j = 0; j <= height; ++j) {
    // j-th line is between cells j-1 and j
    const y = j * CELL_OFFSET + BORDER_SIZE / 2;
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
  }

  ctx.stroke();
};

const getIndex = (row, column) => {
  return row * width + column;
};

const drawCells = () => {
  const cellsPtr = universe.cells_ptr();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; ++row) {
    for (let col = 0; col < width; ++col) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Alive
        ? ALIVE_COLOR
        : DEAD_COLOR;

      ctx.fillRect(
        col * CELL_OFFSET + BORDER_SIZE,
        row * CELL_OFFSET + BORDER_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const update = () => {
  universe.tick();
};

const renderLoop = () => {
  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
}

setInterval(update, 100);
requestAnimationFrame(renderLoop);
