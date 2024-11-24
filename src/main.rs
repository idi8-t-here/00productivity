// main.rs
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use std::env;

mod platform;
use platform::linux::WindowInfo;

#[derive(Debug, Serialize, Deserialize)]
struct TabActivity {
    tab_title: String,
    duration: i64, // Duration in seconds
    opened_time: DateTime<Local>,
    closed_time: DateTime<Local>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Activity {
    application_opened: String,
    duration: i64,
    opened_time: DateTime<Local>,
    closed_time: DateTime<Local>,
    tabs: Vec<TabActivity>, // Assuming you have a TabActivity struct defined
}

struct ActivityTracker {
    previous_window: Option<WindowInfo>,
    previous_time: DateTime<Local>,
}

impl ActivityTracker {
    fn new() -> Self {
        ActivityTracker {
            previous_window: None,
            previous_time: Local::now(),
        }
    }

    fn log_activity(&mut self, window_info: &WindowInfo, tabs: Vec<TabActivity>) {
        let current_time = Local::now();
        let duration = (current_time - self.previous_time).num_seconds();

        if duration > 1 {
            println!("Logging activity for: {:?}", window_info.process_name);
            let activity = Activity {
                application_opened: window_info.process_name.clone(), // Clone here to avoid move
                duration,
                opened_time: self.previous_time,
                closed_time: current_time,
                tabs: vec![],
            };

            // Append the activity to a JSON file
            self.append_to_json_file(activity).expect("Failed to write to log file");
        }

        self.previous_window = Some(window_info.clone()); // Clone the window_info for storage
        self.previous_time = current_time;
    }

    fn append_to_json_file(&self, activity: Activity) -> std::io::Result<()> {
        // Open the file in append mode
        let file_path = "../activity_log.json";
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file_path)?;

        // Serialize the activity to JSON
        let json = serde_json::to_string(&activity)?;

        // Write the JSON object to the file
        writeln!(file, "{}", json)?;
        Ok(())
    }

    



fn run(&mut self) {
    println!("Current directory: {:?}", env::current_dir().unwrap());

    let mut previous_window_info: Option<WindowInfo> = None; // Store previous window info

    loop {
        if let Some(window_info) = platform::linux::get_active_window_info() {
            // Check if the active window has changed
            if previous_window_info.as_ref() != Some(&window_info) {
                // Log closing time for the previous window if it exists
        if let Some(prev_window) = previous_window_info.clone() {
            // Call log_activity and discard the result while logging any errors
                    }

                // Update the previous window info and time
                previous_window_info = Some(window_info.clone());
                self.previous_time = Local::now();
            }
        }
        thread::sleep(Duration::from_secs(1));
    }
}


}


fn main() {
    let tab_activity = TabActivity {
        tab_title: String::from("My Tab"),
        duration: 3600,
        opened_time: Local::now(),
        closed_time: Local::now(),
    };
    println!("{:#?}",{tab_activity});
}
