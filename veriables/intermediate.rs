use winapi::shared::{
    minwindef::{LPARAM, LRESULT, WPARAM},
    windef::POINT,
};
use winapi::um::{
    libloaderapi::GetModuleHandleW,
    winuser::{CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, WH_CALLWNDPROC, WM_SETCURSOR},
};

use std::convert::TryInto;
use winapi::ctypes::c_long;

static mut ORIGINAL_PROC: Option<unsafe extern "system" fn(c_long, WPARAM, LPARAM) -> LRESULT> = None;

unsafe extern "system" fn mouse_hook_proc(n_code: c_long, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == 0 && w_param == WH_CALLWNDPROC {
        let cwp = *(l_param as *const winapi::um::winuser::CWPSTRUCT);
        let msg = cwp.message;
        let hwnd = cwp.hwnd;
        if msg == WM_SETCURSOR {
            let mut cursor_pos = POINT::default();
            winapi::um::winuser::GetCursorPos(&mut cursor_pos);
            let x = cursor_pos.x + 10;
            let y = cursor_pos.y + 10;
            winapi::um::winuser::SetCursorPos(x, y);
            return 1;
        }
    }
    if let Some(proc) = ORIGINAL_PROC {
        proc(n_code, w_param, l_param)
    } else {
        winapi::um::winuser::CallNextHookEx(std::ptr::null_mut(), n_code, w_param, l_param)
    }
}

unsafe extern "system" fn mouse_hook() {
    let module = GetModuleHandleW(std::ptr::null_mut());
    let mut thread_id = 0;
    let hook = SetWindowsHookExW(
        WH_CALLWNDPROC,
        Some(mouse_hook_proc),
        module,
        thread_id.try_into().unwrap(),
    );
    ORIGINAL_PROC = Some(hook);
}

fn main() {
    unsafe {
        mouse_hook();
    }
}