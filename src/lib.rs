mod utils;
mod rom;
mod cpu;

extern crate console_error_panic_hook;
extern crate once_cell;

use wasm_bindgen::prelude::*;
use once_cell::{sync::OnceCell, *};

static mut CPU: OnceCell<cpu::CPU> = OnceCell::new();

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub unsafe fn init() {
    CPU.set(cpu::CPU::new());
}

#[wasm_bindgen]
pub fn tick() {
    unsafe {
        match CPU.get_mut() {
            None => panic!("shouldnt be here"),
            Some(cpu) => cpu.tick()
        }
    }
}

#[wasm_bindgen]
pub unsafe fn stop() {
}
