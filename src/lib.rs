#![feature(once_cell)]
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

#[cfg(feature="const_memory")]
#[global_allocator]
static ALLOCATOR: talc::Talck<talc::locking::AssumeUnlockable, talc::ClaimOnOom> = unsafe {
    static mut MEMORY: [u8; 64 * 1024 * 1024] = [0; 64 * 1024 * 1024];
    let span = talc::Span::from_const_array(std::ptr::addr_of!(MEMORY));
    talc::Talc::new(talc::ClaimOnOom::new(span)).lock()
};

use std::cell::OnceCell;

pub use gui_web::*;
pub use res_mgr_web::*;
pub use pi_spatial::*;
pub use pi_path_finding::*;
pub use pi_orca::*;
use log::info;
pub use pi_bon_decode::*;
pub use pi_export_timer::exports::*;

use wasm_bindgen::prelude::*;
use tracing_log::LogTracer;
use tracing_subscriber::{prelude::*, registry::Registry, EnvFilter, reload::Handle};
use tracing_core::event::Event;

pub struct LogHandle(OnceCell<Handle<EnvFilter, Registry>>);
unsafe impl Sync for LogHandle {}
unsafe impl Send for LogHandle {}
pub static LOG_HANDLE: LogHandle = LogHandle(OnceCell::new());

#[allow(unused_attributes)]
#[wasm_bindgen]
pub fn init_logger(level: pi_web_logger::Level, filter: Option<String>) {
    pi_web_logger::init_with_level(level);
    log::info!("init logger ok");
//     let default_filter = match filter {
//         Some(f) => f,
//         None => "warn".to_string(),
//     };

//     let filter_layer = EnvFilter::try_from_default_env()
//         .or_else(|_| EnvFilter::try_new(&default_filter))
//         .unwrap();
//     let (filter_layer, reload_handle) = tracing_subscriber::reload::Layer::new(filter_layer);
//     let subscriber = Registry::default().with(filter_layer);
//     // app.world.insert_single_res(LogFilterHandle(reload_handle));
//     unsafe {LOG_HANDLE.0.get_or_init(move|| {
//         reload_handle
//     })};

//     console_error_panic_hook::set_once();
//     let mut c_b =  tracing_wasm::WASMLayerConfigBuilder::default();
//     let finished_subscriber = subscriber.with(tracing_wasm::WASMLayer::new(
//         c_b.set_report_logs_in_timings(false).build(),
//     ));

//     let logger_already_set = LogTracer::init().is_err();
//     let subscriber_already_set =
//             tracing::subscriber::set_global_default(finished_subscriber).is_err();

//     match (logger_already_set, subscriber_already_set) {
//         (true, true) => tracing::warn!(
//             "Could not set global logger and tracing subscriber as they are already set. Consider disabling LogPlugin."
//         ),
//         (true, _) => tracing::warn!("Could not set global logger as it is already set. Consider disabling LogPlugin."),
//         (_, true) => tracing::warn!("Could not set global tracing subscriber as it is already set. Consider disabling LogPlugin."),
//         _ => (),
//     };
}

// #[allow(unused_attributes)]
// #[wasm_bindgen]
// pub fn set_log_filter(filter: String) {
//     let handle = unsafe { LOG_HANDLE.0.get().unwrap()};
//     if let Ok(filter_layer) = tracing_subscriber::EnvFilter::try_new(filter) {
//         let _ = handle.modify(|filter| *filter = filter_layer);
//     } else {
        
//     }
// }