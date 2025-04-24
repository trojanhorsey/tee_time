mod ffi;
mod trustonic;
mod brute;
mod menu;

use std::env;

fn main() {
    // Always print the ASCII banner
    menu::print_ascii_menu();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  ./tee_time trustonic <UUID> <TCI bytes...>");
        eprintln!("  ./tee_time brute");
        eprintln!();
        eprintln!("Example:");
        eprintln!("  ./tee_time trustonic 07010000000000000000000000000000 01 ff 00");
        return;
    }

    match args[1].as_str() {
        "trustonic" => {
            if args.len() < 4 {
                eprintln!("❌ Usage: ./tee_time trustonic <UUID> <TCI bytes>");
                return;
            }

            let uuid_str = &args[2];
            let uuid_bytes = match hex::decode(uuid_str) {
                Ok(bytes) if bytes.len() == 16 => bytes,
                _ => {
                    eprintln!("❌ UUID must be 16 hex bytes (no dashes)");
                    return;
                }
            };

            let command_bytes: Vec<u8> = args[3..]
                .iter()
                .filter_map(|b| u8::from_str_radix(b, 16).ok())
                .collect();

            if command_bytes.is_empty() {
                eprintln!("❌ Invalid command bytes.");
                return;
            }

            // Load Trustonic lib (auto-detect path)
            if let Err(e) = ffi::load_trustonic_lib() {
                eprintln!("❌ Failed to load Trustonic lib: {e}");
                return;
            }

            let mut uuid = ffi::McUuid { value: [0u8; 16] };
            uuid.value.copy_from_slice(&uuid_bytes);

            let lib = ffi::trustonic();
            match trustonic::send_tci_command(lib, &uuid, &command_bytes) {
                Ok(resp) => {
                    eprintln!("✅ Response:");
                    for (i, b) in resp.iter().enumerate() {
                        eprint!("{:02x} ", b);
                        if (i + 1) % 16 == 0 {
                            eprintln!();
                        }
                    }
                    eprintln!();
                }
                Err(e) => {
                    eprintln!("❌ TCI command failed: {e}");
                }
            }
        }

        "brute" => {
            brute::run();
        }

        _ => {
            eprintln!("❌ Unknown command: {}", args[1]);
        }
    }
}
