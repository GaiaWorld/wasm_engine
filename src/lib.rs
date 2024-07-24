#![feature(nonnull_slice_from_raw_parts)]
// extern crate gui_web;
// extern crate res_mgr_web;
// extern crate astc_decoder_wasmbindgen;

// extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub use gui_web::*;
pub use res_mgr_web::*;
pub use pi_spatial::*;
pub use pi_path_finding::*;
pub use pi_orca::*;
use log::info;

use wasm_bindgen::prelude::*;

// extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
// use talc::*;
// #[global_allocator]
// static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = {
//     static mut MEMORY: [std::mem::MaybeUninit<u8>; 32 * 1024 * 1024]
//         = [std::mem::MaybeUninit::uninit(); 32 * 1024 * 1024];
//     let span = talc::Span::from_const_array(unsafe { std::ptr::addr_of!(MEMORY) });
//     talc::Talc::new(unsafe { talc::ClaimOnOom::new(span) }).lock()
// };
// static ALLOCATOR: TalckWasm = unsafe { TalckWasm::new_global() };

// use lol_alloc::{FreeListAllocator, LockedAllocator};
// #[global_allocator]
// static ALLOCATOR: LockedAllocator<FreeListAllocator> = LockedAllocator::new(FreeListAllocator::new(67108864));

#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn init_logger(level: pi_web_logger::Level) {
    pi_share::Share::new(1);
    pi_web_logger::init_with_level(level);
	info!("init_logger ok!");
}

// #[wasm_bindgen]
// pub fn init_logger(level: usize) {
//     pi_web_logger::init_with_level(level);
// 	info!("init_logger ok!");
// }

// #[wasm_bindgen]
// pub fn set_string(s: String) -> String {
//     s
// }