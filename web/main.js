const CANVAS_HEIGHT = 512;
const CANVAS_WIDTH = 512;
const BYTES_PER_PIXEL = 3;

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let memory = new WebAssembly.Memory({initial: 17});
let {_module, instance} = await WebAssembly.instantiateStreaming(fetch("wasm-raytracer.wasm"), {env: {memory}});


let len = CANVAS_WIDTH * CANVAS_HEIGHT * BYTES_PER_PIXEL;
let ptr = 0

// index to pixel
let itop = (index) => {
  let offset = index * BYTES_PER_PIXEL;
  return {
    r: bytes[offset],
    g: bytes[offset + 1],
    b: bytes[offset + 2],
  }
};

instance.exports.raytrace(ptr, len);

let bytes = new Uint8Array(memory.buffer, ptr, len);
for (let i = 0; i < CANVAS_WIDTH * CANVAS_HEIGHT; i++) {
  let y = Math.floor(i / CANVAS_WIDTH);
  let x = i % CANVAS_HEIGHT;
  ctx.fillStyle = `rgba(${itop(i).r} ${itop(i).g} ${itop(i).b})`;
  ctx.fillRect(x, y, 1, 1);
}
