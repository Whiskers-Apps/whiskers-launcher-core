use std::{
    fs::{self, File},
    io::Write,
};

use walkdir::WalkDir;

use crate::{
    features::extensions::{ExtensionManifest, ExtensionRequest, FormResponse},
    paths::{
        get_extension_request_path, get_extension_response_path, get_extensions_dir,
        get_form_request_path, get_indexing_extensions_path,
    },
    results::OpenFormAction,
};

use super::settings::{get_settings, write_settings, ExtensionSetting};

pub fn index_extensions() {
    let mut extensions = Vec::<ExtensionManifest>::new();
    let extensions_dir = get_extensions_dir();
    let indexing_extensions_path = get_indexing_extensions_path();
    let mut settings = get_settings();

    if !indexing_extensions_path.parent().unwrap().exists() {
        fs::create_dir_all(&indexing_extensions_path.parent().unwrap())
            .expect("Error creating directory");
    }

    if !extensions_dir.exists() {
        fs::create_dir_all(&extensions_dir).expect("Error creating extensions directory");
    }

    for entry in WalkDir::new(&extensions_dir) {
        if entry.is_ok() {
            let entry = entry.unwrap();
            let name = entry.file_name();

            if name == "manifest.json" {
                let json =
                    fs::read_to_string(entry.path()).expect("Error getting manifest content");

                if let Ok(extension) = serde_json::from_str::<ExtensionManifest>(&json) {
                    extensions.push(extension.to_owned());

                    let has_keyword = settings
                        .extensions
                        .iter()
                        .any(|es| es.extension_id == extension.id && es.setting_id == "keyword");

                    if !has_keyword {
                        settings.extensions.push(ExtensionSetting {
                            extension_id: extension.id.to_owned(),
                            setting_id: String::from("keyword"),
                            setting_value: extension.keyword.to_owned(),
                        })
                    }

                    if let Some(extension_settings) = extension.settings {
                        for extension_setting in extension_settings {
                            let has_setting = settings.extensions.iter().any(|es| {
                                es.extension_id == extension.id
                                    && es.setting_id == extension_setting.id
                            });

                            if !has_setting {
                                settings.extensions.push(ExtensionSetting {
                                    extension_id: extension.id.to_owned(),
                                    setting_id: extension_setting.id.to_owned(),
                                    setting_value: extension_setting.default_value.to_owned(),
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    write_settings(settings);

    let bytes = bincode::serialize(&extensions).expect("Error serializing extensions");
    fs::write(&get_indexing_extensions_path(), &bytes).expect("Error writing extensions");
}

pub fn get_extensions() -> Vec<ExtensionManifest> {
    let path = get_indexing_extensions_path();
    let bytes = fs::read(path).expect("Error reading extensions");

    match bincode::deserialize(&bytes) {
        Ok(extensions) => extensions,
        Err(_) => Vec::new(),
    }
}

pub fn write_extension_request(request: ExtensionRequest) {
    let bytes = bincode::serialize(&request).expect("Error serializing request");

    #[cfg(target_os = "linux")]
    {
        fs::write(get_extension_request_path(), &bytes);
    }

    #[cfg(target_os = "windows")]
    {
        let mut file = File::create(get_extension_request_path()).unwrap();
        file.write_all(&bytes).unwrap();
        file.flush().unwrap();
        file.sync_all().unwrap();
    }
}

pub fn get_extension_request() -> ExtensionRequest {
    let bytes = fs::read(get_extension_request_path()).expect("Error reading extension request");
    let request = bincode::deserialize(&bytes).expect("Error deserializing extension request");
    request
}

pub fn write_form_request(request: OpenFormAction) {
    let bytes = bincode::serialize(&request).expect("Error serializing request");
    fs::write(&get_form_request_path(), &bytes).expect("Error writing request");
}

pub fn get_form_request() -> OpenFormAction {
    let bytes = fs::read(get_form_request_path()).expect("Error reading form request");
    let request = bincode::deserialize(&bytes).expect("Error deserializing form request");
    request
}

pub fn write_form_response(response: FormResponse) {
    let bytes = bincode::serialize(&response).expect("Error serializing response");
    fs::write(get_extension_response_path(), &bytes).expect("Error writing response");
}

pub fn get_form_response() -> FormResponse {
    let bytes = fs::read(get_extension_response_path()).expect("Error reading form response");
    let response = bincode::deserialize(&bytes).expect("Error deserializing form response");
    response
}
