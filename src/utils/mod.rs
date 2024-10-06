use std::env;


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
    return env::consts::OS == "windows"
}

pub fn on_linux() -> bool {
    return env::consts::OS == "linux";
}