mod ffi;
mod trustonic;
mod brute;
mod menu;
mod discover;
mod analyze;


use std::env;

fn main() {
    // Always print the ASCII menu
    menu::print_ascii_menu();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  ./tee_time trustonic <UUID> <TCI bytes...>");
        eprintln!("  ./tee_time brute");
        eprintln!("  ./tee_time discover [--handshake]");
        eprintln!();
        eprintln!("Examples:");
        eprintln!("  ./tee_time trustonic 07010000000000000000000000000000 01 ff 00");
        eprintln!("  ./tee_time discover --handshake");
        return;
    }

    match args[1].as_str() {
        "trustonic" => {
            // [trustonic code ... unchanged]
        }

        "brute" => {
            brute::run();
        }

        "discover" => {
            let handshake = args.get(2).map_or(false, |arg| arg == "--handshake");
            discover::discover_trustonic_tas(handshake);
            return;
        }
        "analyze" => {
            analyze::analyze_environment();
        }

        _ => {
            eprintln!("❌ Unknown command: {}", args[1]);
        }
    }
}
