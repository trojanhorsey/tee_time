use libloading::{Library, Symbol};
use std::sync::Once;

#[repr(C)]
#[derive(Debug)]
pub struct McUuid {
    pub value: [u8; 16],
}

#[repr(C)]
#[derive(Debug)]
pub struct McSessionHandle {
    pub session_id: u32,
    pub device_id: u32,
}

type McOpenDeviceFn = unsafe extern "C" fn(u32) -> i32;
type McCloseDeviceFn = unsafe extern "C" fn(u32) -> i32;
type McOpenSessionFn = unsafe extern "C" fn(*mut McSessionHandle, *const McUuid, *mut u8, u32) -> i32;
type McCloseSessionFn = unsafe extern "C" fn(*mut McSessionHandle) -> i32;
type McNotifyFn = unsafe extern "C" fn(*mut McSessionHandle) -> i32;
type McWaitNotificationFn = unsafe extern "C" fn(*mut McSessionHandle, u32) -> i32;

pub struct TrustonicLib<'lib> {
    _lib: &'lib Library, // hold a reference to keep the library alive
    pub mc_open_device: Symbol<'lib, McOpenDeviceFn>,
    pub mc_close_device: Symbol<'lib, McCloseDeviceFn>,
    pub mc_open_session: Symbol<'lib, McOpenSessionFn>,
    pub mc_close_session: Symbol<'lib, McCloseSessionFn>,
    pub mc_notify: Symbol<'lib, McNotifyFn>,
    pub mc_wait_notification: Symbol<'lib, McWaitNotificationFn>,
}

static INIT: Once = Once::new();
static mut LIB: Option<Box<TrustonicLib<'static>>> = None;

pub fn load_trustonic_lib(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    INIT.call_once(|| {
        // Leak the Library to get a 'static reference
        let lib = Box::leak(Box::new(unsafe {
            Library::new(path).expect("Failed to load libTeeClient")
        }));

        unsafe {
            LIB = Some(Box::new(TrustonicLib {
                mc_open_device: lib.get(b"mcOpenDevice\0").unwrap(),
                mc_close_device: lib.get(b"mcCloseDevice\0").unwrap(),
                mc_open_session: lib.get(b"mcOpenSession\0").unwrap(),
                mc_close_session: lib.get(b"mcCloseSession\0").unwrap(),
                mc_notify: lib.get(b"mcNotify\0").unwrap(),
                mc_wait_notification: lib.get(b"mcWaitNotification\0").unwrap(),
                _lib: lib, // reference to prevent drop
            }));
        }
    });

    Ok(())
}

pub fn trustonic() -> &'static TrustonicLib<'static> {
    unsafe { LIB.as_ref().expect("TrustonicLib not loaded") }
}
