#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use enum2egui_demo::DemoApp;

fn main() -> eframe::Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "enum2egui Demo",
        native_options,
        Box::new(|_cc| Ok(Box::<DemoApp>::default())),
    )
}
