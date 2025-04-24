mod ffi;
mod trustonic;
mod menu;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Always show the banner (to stderr)
    menu::print_ascii_menu();

    // No command provided
    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("  ./tee_time trustonic <UUID> <TCI bytes...>");
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

            // Load Trustonic lib first
            let _ = ffi::load_trustonic_lib("/vendor/lib64/libTeeClient.so")
                .expect("Failed to load Trustonic lib");

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

        _ => {
            eprintln!("❌ Unknown command: {}", args[1]);
        }
    }
}
