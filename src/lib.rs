mod utils;

use std::cmp::{max, min};

use web_sys::js_sys::{Int32Array, Uint8ClampedArray};

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn paint(state: &[usize], width: usize, height: usize) -> Uint8ClampedArray {
    let mut pixels: Vec<u8> = vec![0; width * height * 4];

    let entries = state.chunks(9);

    for entry in entries {
        let x0 = entry[0];
        let y0 = entry[1];
        // let dx = entry[2];
        // let dy = entry[3];
        let size = entry[4];

        let r = entry[5];
        let g = entry[6];
        let b = entry[7];
        let id = entry[8];

        for x in x0..x0 + size {
            for y in y0..y0 + size {
                let in_x = 0 <= x && x < width;
                let in_y = 0 <= y && y < height;

                let index = (y * width + x) * 4;

                if in_x && in_y {
                    pixels[index] = r as u8;
                    pixels[index + 1] = g as u8;
                    pixels[index + 2] = b as u8;
                    pixels[index + 3] = 255;
                }
            }
        }
    }

    Uint8ClampedArray::from(&pixels[..])
}

#[wasm_bindgen]
pub fn update(state: &[i32], width: i32, height: i32, gravity: i32) -> Int32Array {
    let mut next: Vec<i32> = Vec::from(state);

    for i in 0..state.len() / 9 {
        let x0 = state[i * 9];
        let y0 = state[i * 9 + 1];
        let dx = state[i * 9 + 2];
        let dy = state[i * 9 + 3];
        let size = state[i * 9 + 4];

        next[i * 9] = min(width - 1 - size, max(0, x0.wrapping_add(dx)));
        next[i * 9 + 1] = min(height - 1 - size, max(0, y0.wrapping_add(dy)));
        next[i * 9 + 2] = if next[i * 9] + size + 1 == width || next[i * 9] == 0 {
            -dx
        } else {
            dx
        };
        next[i * 9 + 3] = if next[i * 9 + 1] + size + 1 == height {
            -(dy.abs() - dy.abs() / 2)
        } else {
            dy + gravity
        };
    }

    Int32Array::from(&next[..])
}
