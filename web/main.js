import {raytrace} from "./raytracer/wasm_raytracer.js"


const CANVAS_HEIGHT = 512;
const CANVAS_WIDTH = 512;

let bytes = raytrace();

let canvas = document.getElementById("canvas");
let ctx = canvas.getContext("2d");

let imageData = ctx.createImageData(CANVAS_WIDTH, CANVAS_HEIGHT);
for (let i = 0; i < CANVAS_HEIGHT * CANVAS_HEIGHT; i++) {
  let src = i * 3;
  let dst = i * 4;
  imageData.data[dst] = bytes[src];             // r
  imageData.data[dst + 1] = bytes[src + 1];     // g
  imageData.data[dst + 2] = bytes[src + 2];     // b
  imageData.data[dst + 3] = 255;                // a (opaque)
}

ctx.putImageData(imageData, 0, 0);
