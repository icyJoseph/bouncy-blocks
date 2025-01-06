mod utils;

use std::cmp::{max, min};

use wasm_bindgen::prelude::*;

struct Entity {
    x: usize,
    y: usize,
    size: usize,
    color: (u8, u8, u8),
}

impl Entity {
    fn new(desc: &[usize]) -> Self {
        let x = desc[1];
        let y = desc[2];
        let size = desc[5];

        let r = desc[6] as u8;
        let g = desc[7] as u8;
        let b = desc[8] as u8;

        Entity {
            x,
            y,
            size,
            color: (r, g, b),
        }
    }

    fn change_color(&mut self, to: (u8, u8, u8)) -> (u8, u8, u8) {
        let current = self.color;

        self.color = to;

        current
    }

    fn paint(&self, pixels: &mut [u8], width: usize, height: usize) {
        let lower_x = self.x;
        let upper_x = min(lower_x + self.size, width);

        for x in lower_x..upper_x {
            let lower_y = self.y;
            let upper_y = min(lower_y + self.size, height);

            for y in lower_y..upper_y {
                let index = (y * width + x) * 4;

                let (r, g, b) = self.color;

                if let Some(value) = pixels.get_mut(index) {
                    *value = r;
                }

                if let Some(value) = pixels.get_mut(index + 1) {
                    *value = g;
                }

                if let Some(value) = pixels.get_mut(index + 2) {
                    *value = b;
                }

                if let Some(value) = pixels.get_mut(index + 3) {
                    *value = 255;
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn paint(pixels: &mut [u8], state: &[usize], width: usize, height: usize) {
    let entries = state.chunks(9);

    for entry in entries {
        let entity = Entity::new(entry);

        entity.paint(pixels, width, height);
    }
}

#[wasm_bindgen]
pub fn clear(pixels: &mut [u8], state: &[usize], width: usize, height: usize) {
    let entries = state.chunks(9);

    for entry in entries {
        let mut entity = Entity::new(entry);

        let current = entity.change_color((0, 0, 0));

        entity.paint(pixels, width, height);

        entity.change_color(current);
    }
}

struct Block {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    size: i32,
}

impl Block {
    fn new(state: &[i32]) -> Option<Self> {
        let id = state[0];

        if id == 0 {
            return None;
        }

        let x = state[1];
        let y = state[2];
        let dx = state[3];
        let dy = state[4];
        let size = state[5];

        let block = Block { x, y, dx, dy, size };

        Some(block)
    }

    fn update(&mut self, width: i32, height: i32, gravity: i32) {
        let x_bound = width - 1 - self.size;
        let y_bound = height - 1 - self.size;

        self.x = max(0, min(x_bound, self.x + self.dx));
        self.y = max(0, min(y_bound, self.y + self.dy));

        let x_edge = self.x + self.size + 1;

        self.dx = if x_edge == width || self.x == 0 {
            -self.dx
        } else {
            self.dx
        };

        let y_edge = self.y + self.size + 1;

        self.dy = if y_edge == height {
            -(self.dy.abs() - self.dy.abs() / 2)
        } else if self.y == 0 {
            -self.dy
        } else {
            self.dy + gravity
        };
    }

    fn apply(&self, next: &mut [i32]) {
        next[1] = self.x;
        next[2] = self.y;
        next[3] = self.dx;
        next[4] = self.dy;
    }
}

#[wasm_bindgen]
pub fn update(state: &mut [i32], width: i32, height: i32, gravity: i32) {
    for i in 0..state.len() / 9 {
        let from = i * 9;
        let to = from + 9;

        let Some(mut block) = Block::new(&state[from..to]) else {
            return;
        };

        block.update(width, height, gravity);

        block.apply(&mut state[from..to]);
    }
}
