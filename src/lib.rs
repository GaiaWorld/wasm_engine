extern crate gui_web;
extern crate res_mgr_web;
// extern crate astc_decoder_wasmbindgen;

pub use gui_web::*;
pub use res_mgr_web::*;
use log::info;
// pub use astc_decoder_wasmbindgen::*;

use wasm_bindgen::prelude::*;

#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn init_logger(level: pi_web_logger::Level) {
    pi_web_logger::init_with_level(level);
	info!("init_logger ok!");
}
