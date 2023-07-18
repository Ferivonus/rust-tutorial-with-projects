use winapi::shared::{
    minwindef::{LPARAM, LRESULT, WPARAM},
    windef::POINT,
};
use winapi::um::{
    libloaderapi::GetModuleHandleW,
    winuser::{SetWindowsHookExW, WH_CALLWNDPROC, WM_SETCURSOR},
};
use std::convert::TryInto;
use winapi::ctypes::c_long;
use std::thread;
use std::time::Duration;

static mut ORIGINAL_PROC: Option<unsafe extern "system" fn(c_long, WPARAM, LPARAM) -> LRESULT> = None;

unsafe extern "system" fn mouse_hook_proc(n_code: c_long, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == 0 && w_param == WH_CALLWNDPROC as WPARAM {
        let cwp = *(l_param as *const winapi::um::winuser::CWPSTRUCT);
        let msg = cwp.message;
        let _hwnd = cwp.hwnd;
        if msg == WM_SETCURSOR {
            let mut cursor_pos = POINT { x: 0, y: 0 };
            winapi::um::winuser::GetCursorPos(&mut cursor_pos);
            let x = cursor_pos.x;
            let y = cursor_pos.y;
            println!("X: {}, Y: {}", x, y);
        }
    }
    if let Some(proc) = ORIGINAL_PROC {
        proc(n_code, w_param, l_param)
    } else {
        winapi::um::winuser::CallNextHookEx(std::ptr::null_mut(), n_code as i32, w_param, l_param)
    }
}

unsafe extern "system" fn mouse_hook() {
    let module = GetModuleHandleW(std::ptr::null_mut());
    let thread_id = 0;
    let hook = SetWindowsHookExW(
        WH_CALLWNDPROC,
        Some(mouse_hook_proc),
        module,
        thread_id.try_into().unwrap(),
    );
    ORIGINAL_PROC = Some(std::mem::transmute(hook));
}

fn main() {
    unsafe {
        mouse_hook();
    }

    loop {
        // Continuously check the mouse position.
        let mut cursor_pos = POINT { x: 0, y: 0 };
        unsafe {
            winapi::um::winuser::GetCursorPos(&mut cursor_pos);
        }
        let x = cursor_pos.x;
        let y = cursor_pos.y;
        println!("X: {}, Y: {}", x, y);
        thread::sleep(Duration::from_secs(1));
    }
}

/*
    the output of this code:
    X: 1143, Y: 773
    X: 1143, Y: 773
    X: 1143, Y: 773
    X: 957, Y: 867
    X: 939, Y: 928

    Cargo.toml:
    [package]
    name = "rust-tutorial-with-projects"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    winapi = { version = "0.3.9", features = ["winuser", "libloaderapi"] }
*/