use std::path::PathBuf;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentType {
    Agents,
    Commands,
    Contexts,
    Rules,
    Skills,
    Hooks,
    OutputStyles,
    Statusline,
    ConfigFile,
}

impl ComponentType {
    pub fn display_name(&self) -> &str {
        match self {
            Self::Agents => "agents",
            Self::Commands => "commands",
            Self::Contexts => "contexts",
            Self::Rules => "rules",
            Self::Skills => "skills",
            Self::Hooks => "hooks",
            Self::OutputStyles => "output-styles",
            Self::Statusline => "statusline",
            Self::ConfigFile => "config",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InstallStatus {
    New,
    Modified,
    Unchanged,
    Managed,
}

impl InstallStatus {
    pub fn display(&self) -> &str {
        match self {
            Self::New => "new",
            Self::Modified => "modified",
            Self::Unchanged => "installed",
            Self::Managed => "managed",
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct HookConfig {
    pub name: String,
    pub event: String,
    #[serde(rename = "type")]
    pub hook_type: String,
    #[serde(default)]
    pub timeout: Option<u32>,
}

#[derive(Clone, Debug)]
pub struct Component {
    pub component_type: ComponentType,
    pub name: String,
    pub source_path: PathBuf,
    pub dest_path: PathBuf,
    pub selected: bool,
    pub status: InstallStatus,
    pub hook_config: Option<HookConfig>,
}

impl Component {
    pub fn new(
        component_type: ComponentType,
        name: String,
        source_path: PathBuf,
        dest_path: PathBuf,
        status: InstallStatus,
    ) -> Self {
        Self {
            component_type,
            name,
            source_path,
            dest_path,
            selected: status != InstallStatus::Unchanged,
            status,
            hook_config: None,
        }
    }

    pub fn with_hook_config(mut self, config: HookConfig) -> Self {
        self.hook_config = Some(config);
        self
    }

    pub fn display_name(&self) -> String {
        format!("{}/{}", self.component_type.display_name(), self.name)
    }
}
