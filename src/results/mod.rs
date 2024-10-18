use std::{path::PathBuf, vec};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResults {
    pub view_type: SearchViewType,
    pub results: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchViewType {
    Grid,
    List,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub icon: Option<String>,
    pub icon_tint: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub action: ResultAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResultAction {
    pub action_type: ActionType,
    pub dangerous: bool,
    pub copy_text_action: Option<CopyTextAction>,
    pub copy_image_action: Option<CopyImageAction>,
    pub open_link_action: Option<OpenLinkAction>,
    pub open_app_action: Option<OpenAppAction>,
    pub open_form_action: Option<OpenFormAction>,
    pub run_extension_action: Option<RunExtensionAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionType {
    CopyText,
    CopyImage,
    OpenLink,
    OpenApp,
    OpenForm,
    RunExtension,
    DoNothing,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyTextAction {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyImageAction {
    pub image_path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenLinkAction {
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAppAction {
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenFormAction {
    pub extension_id: String,
    pub command: String,
    pub title: String,
    pub fields: Vec<FormField>,
    pub args: Vec<String>,
    pub action_text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunExtensionAction {
    pub extension_id: String,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormField {
    pub id: String,
    pub field_type: FormFieldType,
    pub args: Vec<String>,
    pub input_field: Option<FormInputField>,
    pub text_area_field: Option<FormTextAreaField>,
    pub toggle_field: Option<FormToggleField>,
    pub select_field: Option<FormSelectField>,
    pub file_picker_field: Option<FormFilePickerField>,
    pub folder_picker_field: Option<FormFolderPickerField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FormFieldType {
    Input,
    TextArea,
    Toggle,
    Select,
    FilePicker,
    FolderPicker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormInputField {
    pub title: String,
    pub description: String,
    pub text: String,
    pub placeholder: String,
    pub validation: Option<Vec<FormValidation>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FormValidation {
    IsNumber,
    IsNotEmpty,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormTextAreaField {
    pub title: String,
    pub description: String,
    pub text: String,
    pub placeholder: String,
    pub validation: Option<FormValidation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormToggleField {
    pub title: String,
    pub description: String,
    pub toggled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormSelectField {
    pub title: String,
    pub description: String,
    pub selected_option_id: String,
    pub options: Vec<FormSelectOption>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormSelectOption {
    pub id: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormFilePickerField {
    pub title: String,
    pub description: String,
    pub file_path: Option<String>,
    pub file_types: Option<Vec<String>>,
    pub validation: Option<FormValidation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FormFolderPickerField {
    pub title: String,
    pub description: String,
    pub folder_path: Option<String>,
    pub validation: Option<FormValidation>,
}

impl SearchResults {
    pub fn new_grid_results(results: Vec<SearchResult>) -> Self {
        Self {
            view_type: SearchViewType::Grid,
            results,
        }
    }

    pub fn new_list_results(results: Vec<SearchResult>) -> Self {
        Self {
            view_type: SearchViewType::List,
            results,
        }
    }

    pub fn set_view_type(mut self, view_type: SearchViewType) -> Self {
        self.view_type = view_type;
        self
    }

    pub fn set_results(mut self, results: Vec<SearchResult>) -> Self {
        self.results = results;
        self
    }
}

impl SearchResult {
    pub fn new(title: impl Into<String>, action: ResultAction) -> Self {
        Self {
            icon: None,
            icon_tint: None,
            title: title.into(),
            description: None,
            action,
        }
    }

    pub fn set_icon(mut self, path: PathBuf) -> Self {
        self.icon = Some(path.into_os_string().into_string().unwrap());
        self
    }

    pub fn set_accent_icon_tint(mut self) -> Self {
        self.icon_tint = Some("accent".to_string());
        self
    }

    pub fn set_icon_tint(mut self, tint: impl Into<String>) -> Self {
        self.icon_tint = Some(tint.into());
        self
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn set_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn set_action(mut self, action: ResultAction) -> Self {
        self.action = action;
        self
    }
}

impl ResultAction {
    pub fn new_copy_text_action(action: CopyTextAction) -> Self {
        Self {
            action_type: ActionType::CopyText,
            dangerous: false,
            copy_text_action: Some(action),
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_copy_image_action(action: CopyImageAction) -> Self {
        Self {
            action_type: ActionType::CopyImage,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: Some(action),
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_link_action(action: OpenLinkAction) -> Self {
        Self {
            action_type: ActionType::OpenLink,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: Some(action),
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_app_action(action: OpenAppAction) -> Self {
        Self {
            action_type: ActionType::OpenApp,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: Some(action),
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn new_open_form_action(action: OpenFormAction) -> Self {
        Self {
            action_type: ActionType::OpenForm,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: Some(action),
            run_extension_action: None,
        }
    }

    pub fn new_run_extension_action(action: RunExtensionAction) -> Self {
        Self {
            action_type: ActionType::RunExtension,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: Some(action),
        }
    }

    pub fn new_do_nothing_action() -> Self {
        Self {
            action_type: ActionType::DoNothing,
            dangerous: false,
            copy_text_action: None,
            copy_image_action: None,
            open_link_action: None,
            open_app_action: None,
            open_form_action: None,
            run_extension_action: None,
        }
    }

    pub fn set_dangerous(mut self, dangerous: bool) -> Self {
        self.dangerous = dangerous;
        self
    }
}

impl CopyTextAction {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl CopyImageAction {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = path.into();

        let path_str = path
            .into_os_string()
            .into_string()
            .expect("Error converting image path");

        Self {
            image_path: path_str,
        }
    }
}

impl OpenLinkAction {
    pub fn new(link: impl Into<String>) -> Self {
        Self { link: link.into() }
    }
}

impl OpenAppAction {
    pub fn new(app_id: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
        }
    }
}

impl OpenFormAction {
    pub fn new(
        extension_id: impl Into<String>,
        command: impl Into<String>,
        fields: Vec<FormField>,
    ) -> Self {
        Self {
            extension_id: extension_id.into(),
            command: command.into(),
            title: "Extension Form".to_string(),
            fields,
            action_text: "Ok".to_string(),
            args: vec![],
        }
    }

    pub fn set_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn set_action_text(mut self, text: impl Into<String>) -> Self {
        self.action_text = text.into();
        self
    }

    pub fn add_arg(mut self, arg: impl Into<String>) -> Self {
        let mut new_args = self.args;
        new_args.push(arg.into());
        self.args = new_args;
        self
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

impl RunExtensionAction {
    pub fn new(extension_id: impl Into<String>, command: impl Into<String>) -> Self {
        Self {
            extension_id: extension_id.into(),
            command: command.into(),
            args: vec![],
        }
    }

    pub fn add_arg(mut self, arg: impl Into<String>) -> Self {
        let mut new_args = self.args;
        new_args.push(arg.into());
        self.args = new_args;
        self
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

impl FormField {
    pub fn new_input_field(id: impl Into<String>, field: FormInputField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::Input,
            args: vec![],
            input_field: Some(field),
            text_area_field: None,
            toggle_field: None,
            select_field: None,
            file_picker_field: None,
            folder_picker_field: None,
        }
    }

    pub fn new_text_area_field(id: impl Into<String>, field: FormTextAreaField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::TextArea,
            args: vec![],
            input_field: None,
            text_area_field: Some(field),
            toggle_field: None,
            select_field: None,
            file_picker_field: None,
            folder_picker_field: None,
        }
    }

    pub fn new_toggle_field(id: impl Into<String>, field: FormToggleField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::Toggle,
            args: vec![],
            input_field: None,
            text_area_field: None,
            toggle_field: Some(field),
            select_field: None,
            file_picker_field: None,
            folder_picker_field: None,
        }
    }

    pub fn new_select_field(id: impl Into<String>, field: FormSelectField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::Select,
            args: vec![],
            input_field: None,
            text_area_field: None,
            toggle_field: None,
            select_field: Some(field),
            file_picker_field: None,
            folder_picker_field: None,
        }
    }

    pub fn new_file_picker_field(id: impl Into<String>, field: FormFilePickerField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::FilePicker,
            args: vec![],
            input_field: None,
            text_area_field: None,
            toggle_field: None,
            select_field: None,
            file_picker_field: Some(field),
            folder_picker_field: None,
        }
    }

    pub fn new_folder_picker_field(id: impl Into<String>, field: FormFolderPickerField) -> Self {
        Self {
            id: id.into(),
            field_type: FormFieldType::FolderPicker,
            args: vec![],
            input_field: None,
            text_area_field: None,
            toggle_field: None,
            select_field: None,
            file_picker_field: None,
            folder_picker_field: Some(field),
        }
    }

    pub fn add_arg(mut self, arg: impl Into<String>) -> Self {
        let mut new_args = self.args;
        new_args.push(arg.into());
        self.args = new_args;
        self
    }

    pub fn set_args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }
}

impl FormInputField {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            text: "".to_string(),
            placeholder: "".to_string(),
            validation: None,
        }
    }

    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn set_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn set_not_empty_validation(mut self) -> Self {
        if self.validation.is_some() {
            self.validation = Some(vec![FormValidation::IsNotEmpty, FormValidation::IsNumber]);
        } else {
            self.validation = Some(vec![FormValidation::IsNotEmpty])
        }

        self
    }

    pub fn set_is_number_validation(mut self) -> Self {
        if self.validation.is_some() {
            self.validation = Some(vec![FormValidation::IsNotEmpty, FormValidation::IsNumber]);
        } else {
            self.validation = Some(vec![FormValidation::IsNumber])
        }

        self
    }
}

impl FormTextAreaField {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            text: "".to_string(),
            placeholder: "".to_string(),
            validation: None,
        }
    }

    pub fn set_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn set_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn set_not_empty_validation(mut self) -> Self {
        self.validation = Some(FormValidation::IsNotEmpty);
        self
    }
}

impl FormToggleField {
    pub fn new(title: impl Into<String>, description: impl Into<String>, toggled: bool) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            toggled,
        }
    }
}

impl FormSelectField {
    pub fn new(
        title: impl Into<String>,
        description: impl Into<String>,
        selected_option_id: impl Into<String>,
        options: Vec<FormSelectOption>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            selected_option_id: selected_option_id.into(),
            options,
        }
    }
}

impl FormSelectOption {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
        }
    }
}

impl FormFilePickerField {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            file_path: None,
            file_types: None,
            validation: None,
        }
    }

    pub fn set_file_path(mut self, file_path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = file_path.into();
        let path_str = path
            .into_os_string()
            .into_string()
            .expect("Error getting file path");

        self.file_path = Some(path_str);
        self
    }

    pub fn set_file_types(mut self, file_types: Vec<String>) -> Self {
        self.file_types = Some(file_types);
        self
    }

    /// Sets the file types to the most common image types.
    ///
    /// Those types are **PNG, WEBP, JPG, JPEG**
    pub fn set_image_file_types(mut self) -> Self {
        self.file_types = Some(vec![
            "png".to_string(),
            "webp".to_string(),
            "jpg".to_string(),
            "jpeg".to_string(),
        ]);
        self
    }

    pub fn set_not_empty_validation(mut self) -> Self {
        self.validation = Some(FormValidation::IsNotEmpty);
        self
    }
}

impl FormFolderPickerField {
    pub fn new(title: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            folder_path: None,
            validation: None,
        }
    }

    pub fn set_folder_path(mut self, folder_path: impl Into<PathBuf>) -> Self {
        let path: PathBuf = folder_path.into();
        let path_str = path
            .into_os_string()
            .into_string()
            .expect("Error getting file path");

        self.folder_path = Some(path_str);
        self
    }

    pub fn set_not_empty_validation(mut self) -> Self {
        self.validation = Some(FormValidation::IsNotEmpty);
        self
    }
}
