use std::{fs, path::PathBuf, process::exit};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::{paths::get_extensions_dir, results::SearchResults};

use super::core::settings::get_settings;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionRequest {
    pub extension_id: String,
    pub request_type: ExtensionRequestType,
    pub search_text: Option<String>,
    pub command: Option<String>,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExtensionRequestType {
    GetResults,
    RunCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormResponse {
    pub results: Vec<FormResult>,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormResult {
    pub field_id: String,
    pub field_value: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub keyword: String,
    #[serde(default = "default_settings")]
    pub settings: Option<Vec<ExtensionManifestSetting>>,
    #[serde(default = "default_os")]
    pub os: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionManifestSetting {
    pub id: String,
    pub title: String,
    pub description: String,
    pub setting_type: ExtensionManifestSettingType,
    pub default_value: String,
    #[serde(default = "default_show_conditions")]
    pub show_conditions: Option<Vec<ExtensionManifestShowCondition>>,
    #[serde(default = "default_select_options")]
    pub select_options: Option<Vec<ExtensionManifestSelectOption>>,
    #[serde(default = "default_os")]
    pub os: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ExtensionManifestSettingType {
    Input,
    TextArea,
    Select,
    Toggle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionManifestShowCondition {
    pub setting_id: String,
    pub setting_value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExtensionManifestSelectOption {
    pub id: String,
    pub text: String,
}

impl ExtensionRequest {
    pub fn new_get_results_request(
        extension_id: impl Into<String>,
        search_text: impl Into<String>,
    ) -> Self {
        Self {
            extension_id: extension_id.into(),
            request_type: ExtensionRequestType::GetResults,
            search_text: Some(search_text.into()),
            command: None,
            args: vec![],
        }
    }

    pub fn new_run_command_request(
        extension_id: impl Into<String>,
        command: impl Into<String>,
    ) -> Self {
        Self {
            extension_id: extension_id.into(),
            request_type: ExtensionRequestType::RunCommand,
            search_text: None,
            command: Some(command.into()),
            args: vec![],
        }
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

impl FormResponse {
    pub fn new(results: Vec<FormResult>) -> Self {
        Self {
            results,
            args: vec![],
        }
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

impl FormResult {
    pub fn new(field_id: impl Into<String>, field_value: impl Into<String>) -> Self {
        Self {
            field_id: field_id.into(),
            field_value: field_value.into(),
            args: vec![],
        }
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

fn default_settings() -> Option<Vec<ExtensionManifestSetting>> {
    None
}

fn default_os() -> String {
    "*".to_string()
}

fn default_show_conditions() -> Option<Vec<ExtensionManifestShowCondition>> {
    None
}

fn default_select_options() -> Option<Vec<ExtensionManifestSelectOption>> {
    None
}

pub fn send_search_results(results: SearchResults) {
    let results_json =  serde_json::to_string(&results).expect("Error serializing search results");
    println!("{results_json}");
    exit(0);
}

pub fn get_extension_dir(extension_id: impl Into<String>) -> Option<PathBuf> {
    let extension_id = extension_id.into();
    let extensions_dir = get_extensions_dir();

    for entry in WalkDir::new(&extensions_dir) {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let name = entry.file_name();

            if name == "manifest.json" {
                let json =
                    fs::read_to_string(entry.path()).expect("Error getting manifest content");

                if let Ok(extension) = serde_json::from_str::<ExtensionManifest>(&json) {
                    if extension_id == extension.id {
                        return Some(entry.path().parent().unwrap().to_owned());
                    }
                }
            }
        }
    }

    return None;
}

pub fn get_extension_setting(
    extension_id: impl Into<String>,
    setting_id: impl Into<String>,
) -> Option<String> {
    let setting_id = setting_id.into();
    let extension_id = extension_id.into();

    let settings = get_settings();

    for setting in settings.extensions {
        if setting.extension_id == extension_id && setting.setting_id == setting_id {
            return Some(setting.setting_value);
        }
    }

    None
}