// extern crate gui_web;
// extern crate res_mgr_web;
// extern crate astc_decoder_wasmbindgen;

// extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use lol_alloc::{FreeListAllocator, LockedAllocator};
// #[global_allocator]
// static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new(67108864));

#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = unsafe {
    static mut MEMORY: [u8; 64 * 1024 * 1024] = [0; 64 * 1024 * 1024];
    let span = talc::Span::from_const_array(std::ptr::addr_of!(MEMORY));
    talc::Talc::new(talc::ClaimOnOom::new(span)).lock()
};

pub use gui_web::*;
pub use res_mgr_web::*;
pub use pi_spatial::*;
pub use pi_path_finding::*;
pub use pi_orca::*;
use log::info;
pub use pi_bon_decode::*;

use wasm_bindgen::prelude::*;

#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn init_logger(level: pi_web_logger::Level) {
    pi_web_logger::init_with_level(level);
	info!("init_logger ok!");
}
