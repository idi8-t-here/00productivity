// platform/linux.rs
use std::process::Command;
use xcb::{Connection, x};
use chrono::{DateTime, Local};

#[derive(Debug, PartialEq, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
    pub opened_time: Option<DateTime<Local>>, // Optional field for when the window was opened
    pub closed_time: Option<DateTime<Local>>, // Optional field for when the window was closed
}


pub fn get_active_window_info() -> Option<WindowInfo> {
    let (conn, screen_num) = Connection::connect(None).ok()?;
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize)?;

    // Get active window
    let focus_cookie = conn.send_request(&x::GetInputFocus {});
    let focus_reply = conn.wait_for_reply(focus_cookie).ok()?;
    let active_window: x::Window = focus_reply.focus();

    // Get window title
    let title_cookie = conn.send_request(&x::GetProperty {
        delete: false,
        window: active_window,
        property: x::ATOM_WM_NAME,
        r#type: x::ATOM_STRING,
        long_offset: 0,
        long_length: 1024,
    });

    // Wait for the reply for the window title
    let title_reply = conn.wait_for_reply(title_cookie).ok()?;
    let title = title_reply.value::<u8>()
        .iter()
        .map(|&c| c as char)
        .collect::<String>();

    // Get process name using xprop
    let output = Command::new("xprop")
        .arg("-id")
        .arg(format!("{:?}", active_window))
        .arg("_NET_WM_PID")
        .output()
        .ok()?;

    let pid_str = String::from_utf8_lossy(&output.stdout);
    let pid = pid_str
        .split('=')
        .nth(1)?
        .trim()
        .parse::<u32>()
        .ok()?;

    // Get process name from /proc
    let process_name = std::fs::read_to_string(format!("/proc/{}/comm", pid))
        .ok()?
        .trim()
        .to_string();

    // Return WindowInfo with None for opened_time and closed_time
    Some(WindowInfo {
        title,
        process_name,
        opened_time: None, // Initialize as None
        closed_time: None, // Initialize as None
    })
}

