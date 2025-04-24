use crate::ffi::{self, McSessionHandle, McUuid};
use hex::encode;
use std::ptr;

pub fn run() {
    let lib = match ffi::load_trustonic_lib() {
        Ok(_) => ffi::trustonic(),
        Err(_) => return,
    };

    let _ = unsafe { (lib.mc_open_device)(0) };

    for a in 0x00..=0xFFu8 {
        for b in 0x00..=0xFFu8 {
            let mut uuid_raw = [0u8; 16];
            uuid_raw[0] = a;
            uuid_raw[1] = b;

            let uuid = McUuid { value: uuid_raw };

            let mut session = McSessionHandle {
                session_id: 0,
                device_id: 0,
            };

            let result = unsafe {
                (lib.mc_open_session)(
                    &mut session,
                    &uuid,
                    ptr::null_mut(),
                    0,
                )
            };

            if result == 0 {
                let uuid_str = encode(uuid_raw);
                eprintln!("[*] UUID {} → ✅ Loadable", uuid_str);
                unsafe { (lib.mc_close_session)(&mut session) };
            }
        }
    }

    let _ = unsafe { (lib.mc_close_device)(0) };
}
