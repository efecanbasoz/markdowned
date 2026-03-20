use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Ollama,
    Google,
    #[serde(rename = "openai")]
    OpenAI,
    Anthropic,
    Custom,
}

impl Default for Provider {
    fn default() -> Self {
        Provider::Ollama
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionConfig {
    #[serde(default)]
    pub provider: Provider,
    #[serde(default)]
    pub api_key: String,
    #[serde(default)]
    pub api_key_in_keychain: bool,
    #[serde(default)]
    pub model: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub custom: CustomProviderConfig,
    #[serde(default)]
    pub auto_completion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomProviderConfig {
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub completion: CompletionConfig,
    #[serde(default)]
    pub last_workspace: Option<String>,
    #[serde(default = "default_split_direction")]
    pub split_direction: String,
    #[serde(default = "default_theme")]
    pub theme: String,
}

fn default_split_direction() -> String {
    "horizontal".to_string()
}

fn default_theme() -> String {
    "dark".to_string()
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            provider: Provider::Ollama,
            api_key: String::new(),
            api_key_in_keychain: false,
            model: String::new(),
            base_url: String::new(),
            custom: CustomProviderConfig::default(),
            auto_completion: false,
        }
    }
}

impl CompletionConfig {
    pub fn resolve_api_key(&self) -> String {
        if self.api_key_in_keychain {
            if let Ok(Some(key)) = crate::services::keychain::retrieve_api_key(
                &format!("{:?}", self.provider).to_lowercase(),
            ) {
                return key;
            }
        }
        self.api_key.clone()
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            completion: CompletionConfig::default(),
            last_workspace: None,
            split_direction: default_split_direction(),
            theme: default_theme(),
        }
    }
}

impl AppConfig {
    pub fn config_path() -> std::path::PathBuf {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("markdowned");
        std::fs::create_dir_all(&config_dir).ok();
        config_dir.join("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        let content = toml::to_string_pretty(self).map_err(|e| e.to_string())?;
        std::fs::write(&path, content).map_err(|e| e.to_string())?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(&path, perms).ok();
        }
        Ok(())
    }
}
