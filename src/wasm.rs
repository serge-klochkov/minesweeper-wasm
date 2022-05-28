use wasm_bindgen::prelude::*;
use crate::minesweeper::{Cell, Minesweeper};
use std::cell::RefCell;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper> = RefCell::new(Minesweeper::new(10, 10, 8))
}

#[wasm_bindgen(js_name = getWidth)]
pub fn get_width() -> usize {
    MINESWEEPER.with(|ms| {
        ms.borrow().width
    })
}

#[wasm_bindgen(js_name = getHeight)]
pub fn get_height() -> usize {
    MINESWEEPER.with(|ms| {
        ms.borrow().height
    })
}

#[wasm_bindgen(js_name = toggleFlag)]
pub fn toggle_flag(x: usize, y: usize) -> bool {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().toggle_flag((x, y))
    })
}

#[wasm_bindgen(js_name = getCells)]
pub fn get_cells() -> Vec<usize> {
    MINESWEEPER.with(|ms| {
        ms.borrow().cells.iter().map(|c| {
            match c {
                Cell::Closed(_) => 0,
                Cell::Flagged(_) => 10,
                Cell::Open(count) => 20 + count, // max 8 mines around, so 20..=28
                Cell::RevealedMine => 30,
            }
        }).collect()
    })
}

#[wasm_bindgen(js_name = openCell)]
pub fn open_cell(x: usize, y: usize) -> bool {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().open((x, y))
    })
}

#[wasm_bindgen]
pub fn restart() {
    MINESWEEPER.with(|ms| {
        ms.borrow_mut().restart();
    })
}