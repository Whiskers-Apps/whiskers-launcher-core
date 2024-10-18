#[cfg(target_os = "linux")]
use {
    freedesktop_desktop_entry::{default_paths, DesktopEntry, Iter},
    tux_icons::icon_fetcher::IconFetcher,
};

use {
    crate::paths::{get_indexing_apps_path, get_indexing_dir, get_indexing_icons_dir},
    serde::{Deserialize, Serialize},
    std::{fs, process::Command},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct App {
    pub id: String,
    pub title: String,
    pub icon: Option<String>,
    pub path: String,
}

impl App {
    pub fn new(id: impl Into<String>, title: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            icon: None,
            path: path.into(),
        }
    }

    pub fn set_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

/// Gets the apps from the system and indexes them into a file
pub fn index_apps() {
    #[cfg(target_os = "linux")]
    if cfg!(target_os = "linux") {
        let mut apps_indexing = Vec::<App>::new();
        let mut ids = Vec::<String>::new();
        let fetcher = IconFetcher::new().set_return_target_path(true);
        let icons_dir = get_indexing_icons_dir();

        if !icons_dir.exists() {
            fs::create_dir_all(&icons_dir).expect("Error creating icons directory");
        }

        //Gets All Apps
        for path in Iter::new(default_paths()) {
            if let Ok(bytes) = fs::read_to_string(&path) {
                if let Ok(entry) = DesktopEntry::decode(&path, &bytes) {
                    if !ids.contains(&entry.appid.to_string()) && !entry.no_display() {
                        ids.push(entry.appid.to_string());

                        if let Some(entry_type) = entry.type_() {
                            if entry_type == "Application" && !entry.no_display() {}
                        }

                        if entry.type_().unwrap() == "Application" && !entry.no_display() {
                            let icon = fetcher.get_icon_path_from_desktop(entry.path);
                            let exec_path = path.clone().into_os_string().into_string().unwrap();
                            let title = entry.name(None).unwrap().to_string();

                            let id_command = Command::new("sh")
                                .arg("-c")
                                .arg(format!("ls -i {}", &exec_path))
                                .output()
                                .expect("Error executing ls -l");

                            let id_command_out =
                                String::from_utf8_lossy(&id_command.stdout).to_string();

                            let split_command: Vec<&str> = id_command_out.split(" ").collect();

                            let id = split_command.get(0).unwrap().to_owned().to_owned();

                            if !id.trim().is_empty() {
                                let mut app_indexing = App::new(&id, &title, &exec_path);

                                match icon.clone() {
                                    Some(path) => {
                                        let icon_path_str =
                                            path.to_owned().into_os_string().into_string().unwrap();

                                        if !icon_path_str.ends_with(".svgz") {
                                            let file_type = path
                                                .extension()
                                                .unwrap()
                                                .to_os_string()
                                                .into_string()
                                                .unwrap();

                                            let mut index_icon_path = icons_dir.to_owned();
                                            index_icon_path.push(format!("{id}.{file_type}"));

                                            fs::copy(&path, &index_icon_path)
                                                .expect("Error copying icon");

                                            app_indexing = app_indexing.set_icon(icon_path_str);
                                        }
                                    }
                                    None => {}
                                }

                                apps_indexing.push(app_indexing);
                            }
                        }
                    }
                }
            }
        }

        let indexig_dir = get_indexing_dir();

        if !indexig_dir.exists() {
            fs::create_dir_all(&indexig_dir).expect("Error creating indexing dir");
        }

        apps_indexing.sort_by_key(|a| a.to_owned().title);

        let encoded_apps_indexing =
            bincode::serialize(&apps_indexing).expect("Error serializing apps indexing");

        fs::write(get_indexing_apps_path(), encoded_apps_indexing)
            .expect("Error writing apps indexing");
    }

    #[cfg(target_os = "windows")]
    if cfg!(target_os = "windows") {
        let mut script_path = get_app_resources_dir();
        script_path.push("scripts/index-apps.ps1");

        let script_content =
            fs::read_to_string(&script_path).expect("Error getting script content");
        powershell_script::run(&script_content).expect("Error running index script");

        let mut apps_json_path = get_app_dir();
        apps_json_path.push("indexing/apps.json");

        let apps_json_content =
            fs::read_to_string(&apps_json_path).expect("Error getting apps json");

        let apps: Vec<App> = serde_json::from_str(&apps_json_content).expect("Error getting apps");

        let bytes = bincode::serialize(&apps).expect("Error serializing apps");

        if !get_indexing_dir().exists() {
            fs::create_dir_all(get_indexing_dir()).expect("Error creating index directory");
        }

        fs::write(get_indexing_apps_path(), &bytes).expect("Error writing apps binary");
    }
}

pub fn get_apps() -> Vec<App> {
    let bytes = fs::read(get_indexing_apps_path()).expect("Error reading indexing apps");

    match bincode::deserialize(&bytes) {
        Ok(apps_indexing) => apps_indexing,
        Err(_) => Vec::new(),
    }
}
