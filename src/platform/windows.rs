// platform/windows.rs
#[cfg(windows)]
use windows_sys::Win32::{
    Foundation::{HWND, MAX_PATH},
    UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId},
    System::{
        Diagnostics::Debug::K32GetModuleBaseNameW,
        Threading::OpenProcess,
        ProcessStatus::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

#[cfg(windows)]
#[derive(Debug, PartialEq)]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
}

#[cfg(windows)]
pub fn get_active_window_info() -> Option<WindowInfo> {
    unsafe {
        // Get handle of the foreground window
        let hwnd = GetForegroundWindow();
        if hwnd == 0 {
            return None;
        }

        // Get window title
        let mut title = [0u16; 512];
        let len = GetWindowTextW(hwnd, title.as_mut_ptr(), title.len() as i32);
        if len == 0 {
            return None;
        }
        let window_title = String::from_utf16_lossy(&title[..len as usize]);

        // Get process ID associated with the window
        let mut process_id = 0;
        GetWindowThreadProcessId(hwnd, &mut process_id);

        // Open the process to retrieve its name
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
        if process_handle == 0 {
            return None;
        }

        // Get the process name
        let mut exe_name = [0u16; MAX_PATH as usize];
        let len = K32GetModuleBaseNameW(process_handle, 0, exe_name.as_mut_ptr(), exe_name.len() as u32);
        if len == 0 {
            return None;
        }
        let process_name = String::from_utf16_lossy(&exe_name[..len as usize]);

        Some(WindowInfo {
            title: window_title,
            process_name,
        })
    }
}
