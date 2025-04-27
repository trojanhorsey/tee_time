use std::fs;
use std::path::Path;

/// Common folders to scan
const COMMON_TA_PATHS: &[&str] = &[
    "/vendor/firmware",
    "/vendor/firmware/mcRegistry",
    "/vendor/app/mcRegistry",
    "/system/etc",
];

/// Discover TAs by scanning known folders
pub fn discover_trustonic_tas() {
    println!("🔍 Discovering Trustonic TAs...");

    for base_path in COMMON_TA_PATHS {
        if let Ok(entries) = fs::read_dir(base_path) {
            println!("Scanning {}...", base_path);

            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    // Match UUID filenames
                    if filename.ends_with(".tlbin") && filename.len() == 32 + 6 {
                        let uuid = &filename[..32];
                        println!("📦 Found TA: {} (from {})", uuid, base_path);
                    }
                }
            }
        }
    }
}
