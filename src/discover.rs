use crate::ffi::{McSessionHandle, McUuid, trustonic, load_trustonic_lib};
use std::fs;
use std::path::Path;

pub fn discover_trustonic_tas(handshake: bool) {
    println!("🔎 Discovering Trustonic TAs...");

    let paths = [
        "/vendor/app/mcRegistry",
        "/data/app/mcRegistry",
        "/vendor/firmware",
    ];

    let mut found = Vec::new();

    for path in paths.iter() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.ends_with(".tlbin") || name.ends_with(".mclf") {
                        println!("📦 Found TA: {}", name);

                        if let Some(uuid_bytes) = guess_uuid_from_filename(name) {
                            found.push(uuid_bytes);
                        }
                    }
                }
            }
        }
    }

    if found.is_empty() {
        println!("❌ No Trustonic TAs found.");
        return;
    }

    if !handshake {
        println!("✅ Discovery complete. {} UUID(s) found.", found.len());
        return; // <-- **Just return cleanly, no printing all UUIDs**
    }

    // If handshake was requested:
    println!("🤝 Handshaking with discovered TAs...");

    if let Err(e) = load_trustonic_lib() {
        eprintln!("❌ Failed to load Trustonic lib: {e}");
        return;
    }

    let lib = trustonic();

    for uuid in found {
        let mut mc_uuid = McUuid { value: [0u8; 16] };
        mc_uuid.value.copy_from_slice(&uuid);

        match try_handshake(lib, &mc_uuid) {
            Ok(resp) => {
                println!("✅ UUID {} responded! First 64 bytes:", uuid_to_string(&uuid));
                for (i, b) in resp.iter().enumerate() {
                    print!("{:02x} ", b);
                    if (i + 1) % 16 == 0 {
                        println!();
                    }
                }
                println!();
            }
            Err(e) => {
                println!("❌ UUID {} handshake failed: {}", uuid_to_string(&uuid), e);
            }
        }
    }
}

/// Try opening session + basic handshake
fn try_handshake(lib: &crate::ffi::TrustonicLib, uuid: &McUuid) -> Result<Vec<u8>, String> {
    unsafe {
        (lib.mc_open_device)(0);

        let mut tci_buffer = [0u8; 4096];
        tci_buffer[0] = 0x00; // very basic handshake

        let mut session = McSessionHandle { session_id: 0, device_id: 0 };

        let open_result = (lib.mc_open_session)(
            &mut session,
            uuid as *const McUuid,
            tci_buffer.as_mut_ptr(),
            tci_buffer.len() as u32,
        );

        if open_result != 0 {
            return Err(format!("mcOpenSession failed ({})", open_result));
        }

        (lib.mc_notify)(&mut session);
        let wait_result = (lib.mc_wait_notification)(&mut session, 5000);

        (lib.mc_close_session)(&mut session);
        (lib.mc_close_device)(0);

        if wait_result != 0 {
            return Err(format!("mcWaitNotification failed ({})", wait_result));
        }

        Ok(tci_buffer[..64].to_vec())
    }
}

/// Try to guess UUID bytes from filename
fn guess_uuid_from_filename(name: &str) -> Option<Vec<u8>> {
    let stripped = name.replace("-", "").replace(".tlbin", "").replace(".mclf", "");
    if stripped.len() == 32 {
        hex::decode(stripped).ok()
    } else {
        None
    }
}

/// UUID bytes to readable string
fn uuid_to_string(uuid: &[u8]) -> String {
    uuid.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("")
}
