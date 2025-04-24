use std::io::{stdin, stdout, Write};
use crate::ffi::{McSessionHandle, McUuid, TrustonicLib, trustonic};

/// Send a TCI command and return the response buffer
pub fn send_tci_command(lib: &TrustonicLib, uuid: &McUuid, command: &[u8]) -> Result<Vec<u8>, String> {
    unsafe {
        // Open device
        let dev_result = (lib.mc_open_device)(0);
        if dev_result != 0 {
            return Err(format!("mcOpenDevice failed with code {}", dev_result));
        }

        // Prepare buffer
        let mut tci_buffer = [0u8; 4096];
        if command.len() > tci_buffer.len() {
            return Err("Command too large for TCI buffer".into());
        }
        tci_buffer[..command.len()].copy_from_slice(command);

        let mut session = McSessionHandle { session_id: 0, device_id: 0 };

        // Open session
        let open_result = (lib.mc_open_session)(
            &mut session,
            uuid as *const McUuid,
            tci_buffer.as_mut_ptr(),
            tci_buffer.len() as u32,
        );        

        if open_result != 0 {
            return Err(format!("mcOpenSession failed with code {}", open_result));
        }

        // Notify
        let notify_result = (lib.mc_notify)(&mut session);
        if notify_result != 0 {
            return Err(format!("mcNotify failed: {}", notify_result));
        }

        // Wait
        let wait_result = (lib.mc_wait_notification)(&mut session, 5000);
        if wait_result != 0 {
            return Err(format!("mcWaitNotification failed: {}", wait_result));
        }

        // Close session and device
        (lib.mc_close_session)(&mut session);
        (lib.mc_close_device)(0);

        // Return first 64 bytes of response
        Ok(tci_buffer[..64].to_vec())
    }
}

/// Interactive TEE menu
pub fn trustonic_menu() {
    let lib = trustonic();

    let mut session = McSessionHandle { session_id: 0, device_id: 0 };
    let mut tci_buffer = [0u8; 4096];
    let mut uuid = McUuid { value: [0; 16] };

    loop {
        println!("\n[Trustonic TEE Interface]");
        println!("[1] Open session to a UUID");
        println!("[2] Send TCI command");
        println!("[3] Wait for notification");
        println!("[4] Dump response buffer");
        println!("[b] Back");

        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                print!("Enter UUID hex (16 bytes, no dashes, e.g. 07010000000000000000000000000000): ");
                stdout().flush().unwrap();
                input.clear();
                stdin().read_line(&mut input).unwrap();
                let bytes = hex::decode(input.trim()).unwrap_or_default();
                if bytes.len() == 16 {
                    uuid.value.copy_from_slice(&bytes);
                    let open_result = unsafe {
                        (lib.mc_open_device)(0);
                        (lib.mc_open_session)(
                            &mut session,
                            &uuid,
                            tci_buffer.as_mut_ptr(),
                            tci_buffer.len() as u32,
                        )
                    };
                    if open_result == 0 {
                        println!("✅ Session opened successfully.");
                    } else {
                        println!("❌ Failed to open session: {}", open_result);
                    }
                } else {
                    println!("❌ Invalid UUID format.");
                }
            }

            "2" => {
                print!("Enter command bytes (e.g. 01 ff 00): ");
                stdout().flush().unwrap();
                input.clear();
                stdin().read_line(&mut input).unwrap();
                let command_bytes: Vec<u8> = input
                    .trim()
                    .split_whitespace()
                    .filter_map(|x| u8::from_str_radix(x, 16).ok())
                    .collect();

                if command_bytes.is_empty() {
                    println!("❌ Invalid command input.");
                    continue;
                }

                match send_tci_command(lib, &uuid, &command_bytes) {
                    Ok(response) => {
                        println!("✅ Command sent. Response (first 64 bytes):");
                        for (i, byte) in response.iter().enumerate() {
                            print!("{:02x} ", byte);
                            if (i + 1) % 16 == 0 {
                                println!();
                            }
                        }
                        println!();
                    }
                    Err(e) => println!("❌ Error sending TCI command: {}", e),
                }
            }

            "3" => {
                println!("⏳ Waiting for notification...");
                let res = unsafe { (lib.mc_wait_notification)(&mut session, 5000) };
                if res == 0 {
                    println!("📬 Notification received.");
                } else {
                    println!("❌ Wait failed: {}", res);
                }
            }

            "4" => {
                println!("📤 Dumping response buffer (first 64 bytes):");
                for (i, byte) in tci_buffer.iter().take(64).enumerate() {
                    print!("{:02x} ", byte);
                    if (i + 1) % 16 == 0 {
                        println!();
                    }
                }
                println!();
            }

            "b" => break,

            _ => println!("Invalid input."),
        }
    }

    unsafe {
        (lib.mc_close_session)(&mut session);
        (lib.mc_close_device)(0);
    }
}
