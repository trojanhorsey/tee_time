use std::fs;

pub fn analyze_environment() {
    println!("🔎 Analyzing device environment...");

    // 1. Root Check
    if is_root() {
        println!("✅ Root access: Yes");
    } else {
        println!("❌ Root access: No");
    }

    // 2. Hardware Info
    if let Some(hardware) = get_hardware_info() {
        println!("📱 Hardware: {}", hardware);
    } else {
        println!("❌ Failed to get hardware info.");
    }

    // 3. TEE Vendor
    if let Some(vendor) = detect_tee_vendor() {
        println!("🔐 TEE Vendor Detected: {}\n", vendor);
    } else {
        println!("❌ TEE Vendor could not be determined.\n");
    }
}

fn is_root() -> bool {
    fs::metadata("/data/local/tmp").is_ok()
}

fn get_hardware_info() -> Option<String> {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("Hardware") {
                return line.split(':').nth(1).map(|s| s.trim().to_string());
            }
        }
    }
    None
}

fn detect_tee_vendor() -> Option<String> {
    let candidates = [
        ("/vendor/bin/tee", "Trustonic"),
        ("/system/bin/tee", "Trustonic"),
        ("/vendor/lib/libMcTeeClient.so", "Trustonic"),
        ("/vendor/lib64/libMcTeeClient.so", "Trustonic"),
        ("/vendor/lib/libQSEEComAPI.so", "Qualcomm QSEE"),
        ("/vendor/lib64/libQSEEComAPI.so", "Qualcomm QSEE"),
        ("/vendor/lib/libtee.so", "MediaTek TEE"),
        ("/vendor/lib64/libtee.so", "MediaTek TEE"),
    ];

    for (path, vendor) in candidates.iter() {
        if fs::metadata(path).is_ok() {
            return Some(vendor.to_string());
        }
    }

    None
}
