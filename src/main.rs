#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::app::App;

mod gui;
mod app;
mod model;
mod app_state;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    const SOFT_NAME: &str = "Osiris genealogy";
    let version: &str = option_env!("CARGO_PKG_VERSION").unwrap_or("");

    eframe::run_native(
        &format!("{SOFT_NAME} {version}"),
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )


}