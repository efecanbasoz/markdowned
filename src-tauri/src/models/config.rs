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
    pub model: String,
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub custom: CustomProviderConfig,
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
}

impl Default for CompletionConfig {
    fn default() -> Self {
        Self {
            provider: Provider::Ollama,
            api_key: String::new(),
            model: String::new(),
            base_url: String::new(),
            custom: CustomProviderConfig::default(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            completion: CompletionConfig::default(),
            last_workspace: None,
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
        std::fs::write(&path, content).map_err(|e| e.to_string())
    }
}
