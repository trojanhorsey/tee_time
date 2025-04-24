// src/main.rs
mod menu;
mod trustonic;
mod ffi;

use crate::ffi::load_trustonic_lib;

fn main() {
    println!("\u{1f525} TEEtime launched!");

    if let Err(e) = load_trustonic_lib("/vendor/lib64/libTeeClient.so") {
        eprintln!("\u{274c} Failed to load lib: {:?}", e);
        return;
    }

    println!("\u{2705} lib loaded. Launching menu...");
    menu::main_menu_from_args();
}
