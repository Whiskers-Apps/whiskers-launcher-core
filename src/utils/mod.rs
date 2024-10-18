use std::env;

use notify_rust::Notification;

/// Makes the command run without a terminal window
pub const FLAG_NO_WINDOW: u32 = 0x08000000;

/// Makes the command run in a thread
pub const FLAG_DETACHED_PROCESS: u32 = 0x00000008;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub keyword: Option<String>,
    pub search_text: String,
}

pub fn get_search_query(search_input: impl Into<String>) -> SearchQuery {
    let search_input = search_input.into();

    let mut keyword = "".to_string();
    let mut search_text = "".to_string();
    let mut has_keyword = false;

    for char in search_input.chars() {
        if char == ' ' && !has_keyword {
            has_keyword = true;
        } else if !has_keyword {
            keyword += &char.to_string();
        } else {
            search_text += &char.to_string();
        }
    }

    if !has_keyword {
        search_text = keyword.to_owned();
    }

    search_text = search_text.trim().to_string();

    SearchQuery {
        keyword: match has_keyword {
            true => Some(keyword),
            false => None,
        },
        search_text,
    }
}

pub fn on_windows() -> bool {
    return env::consts::OS == "windows";
}

pub fn on_linux() -> bool {
    return env::consts::OS == "linux";
}

pub fn on_wayland() -> bool {
    if let Ok(display_server) = env::var("XDG_SESSION_TYPE") {
        return display_server.to_lowercase() == "wayland";
    }

    false
}

pub fn on_hyprland() -> bool {
    if let Ok(environment) = env::var("XDG_CURRENT_DESKTOP") {
        if environment.to_lowercase() == "hyprland" {
            return true;
        }
    }

    false
}

pub fn send_notification(title: impl Into<String>, description: impl Into<String>) {
    let title = title.into();
    let description = description.into();

    #[cfg(target_os = "linux")]
    {
        Notification::new()
            .summary(&title)
            .body(&description)
            .icon("whiskers-launcher")
            .show()
            .expect("Error sending notification");
    }
}