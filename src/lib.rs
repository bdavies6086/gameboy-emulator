mod utils;
mod rom;

extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use std::str;
use rom::*;
use utils::set_panic_hook;

static mut HALT: bool = false;


#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn init() {
    set_panic_hook();
    let cb = rom::initialise_rom();

    let s = cb(1).to_string();
    
    log(&s);

    let a = cb(0).to_string();

    log(&a);
    alert("hi");
}

#[wasm_bindgen]
pub fn stop() {
    unsafe {
        HALT = true;
    }
    
    alert("ive been stopped");
}
