use dirs::home_dir;
use std::path::{Path, PathBuf};
use std::env;

pub fn get_app_dir() -> PathBuf {
    match env::consts::OS {
        "windows" => {
            let mut path = Path::new(&env::var("APPDATA").unwrap()).to_owned();
            path.push("com-whiskersapps-launcher");
            path
        }
        _ => {
            let mut path = get_home_dir();
            path.push(".local/share/com-whiskersapps-launcher");
            path
        }
    }
}

pub fn get_app_resources_dir() -> PathBuf {
    let mut path = get_app_dir();
    path.push("resources");
    path
}

pub fn get_app_resources_icons_dir() -> PathBuf {
    let mut path = get_app_resources_dir();
    path.push("icons");
    path
}

pub fn get_indexing_dir() -> PathBuf {
    match env::consts::OS {
        "windows" => {
            let mut path = get_home_dir();
            path.push(".whiskers-launcher/indexing");
            path
        }
        _ => {
            let mut path = get_home_dir();
            path.push(".cache/whiskers-launcher-indexing");
            path
        }
    }
}

pub fn get_indexing_shortcuts_dir() -> PathBuf {
    let mut path = get_indexing_dir();
    path.push("shortcuts");
    path
}

pub fn get_indexing_icons_dir() -> PathBuf {
    let mut path = get_indexing_dir();
    path.push("icons");
    path
}

pub fn get_indexing_shortcuts_path() -> PathBuf {
    let mut path = get_indexing_dir();
    path.push("shortcuts.bin");
    path
}

pub fn get_indexing_extensions_path() -> PathBuf {
    let mut path = get_indexing_dir();
    path.push("extensions.bin");
    path
}

pub fn get_indexing_apps_path() -> PathBuf {
    let mut path = get_indexing_dir();
    path.push("apps.bin");
    path
}

pub fn get_api_dir() -> PathBuf {
    match env::consts::OS {
        "windows" => {
            let mut path = Path::new(&env::var("TEMP").unwrap()).to_owned();
            path.push("whiskers-launcher-api");
            path
        }
        _ => Path::new("/tmp/whiskers-launcher-api").to_owned(),
    }
}

pub fn get_home_dir() -> PathBuf {
    home_dir().unwrap()
}

pub fn get_extension_response_path() -> PathBuf {
    let mut path = get_api_dir();
    path.push("extension-response.bin");
    path
}

pub fn get_extension_request_path() -> PathBuf {
    let mut path = get_api_dir();
    path.push("extension-request.bin");
    path
}

pub fn get_form_response_path() -> PathBuf {
    let mut path = get_api_dir();
    path.push("form-response.bin");
    path
}

pub fn get_form_request_path() -> PathBuf {
    let mut path = get_api_dir();
    path.push("form-request.bin");
    path
}

pub fn get_extensions_dir() -> PathBuf {
    let mut path = get_app_dir();
    path.push("extensions");
    path
}

pub fn get_settings_path() -> PathBuf {
    match env::consts::OS {
        "windows" => {
            let mut path = get_app_dir();
            path.push("settings.bin");
            path
        }
        _ => {
            let mut path = get_home_dir();
            path.push(".config/whiskers-launcher/settings.bin");
            path
        }
    }
}

pub fn get_autostart_dir() -> PathBuf {
    match env::consts::OS {
        "windows" => {
            let mut path =
                Path::new(&env::var("APPDATA").expect("Error getting APPDATA directory"))
                    .to_owned();
            path.push("Microsoft\\Windows\\Start Menu\\Programs\\Startup");
            path
        }
        _ => {
            let mut path = get_home_dir();
            path.push(".config/autostart");
            return path;
        }
    }
}

pub fn get_stores_cache_dir() -> PathBuf {
    let mut path = get_app_dir();
    path.push("stores-cache");
    path
}

pub fn get_extensions_store_path() -> PathBuf{
    let mut path = get_stores_cache_dir();
    path.push("extensions.bin");
    path
}

pub fn get_themes_store_path() -> PathBuf{
    let mut path = get_stores_cache_dir();
    path.push("themes.bin");
    path
}

pub fn get_recent_apps_path() -> PathBuf{
    let mut path = get_app_dir();
    path.push("recent-apps.bin");
    path
}