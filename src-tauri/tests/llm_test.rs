use markdowned_lib::models::config::{CompletionConfig, CustomProviderConfig, Provider};
use markdowned_lib::services::llm;

fn default_config(provider: Provider) -> CompletionConfig {
    CompletionConfig {
        provider,
        api_key: String::new(),
        model: String::new(),
        base_url: String::new(),
        custom: CustomProviderConfig::default(),
    }
}

#[test]
fn test_resolve_base_url_ollama() {
    let cfg = default_config(Provider::Ollama);
    assert_eq!(llm::resolve_base_url(&cfg), "http://localhost:11434/v1");
}

#[test]
fn test_resolve_base_url_openai() {
    let cfg = default_config(Provider::OpenAI);
    assert_eq!(llm::resolve_base_url(&cfg), "https://api.openai.com/v1");
}

#[test]
fn test_resolve_base_url_google() {
    let cfg = default_config(Provider::Google);
    assert!(llm::resolve_base_url(&cfg).contains("generativelanguage.googleapis.com"));
}

#[test]
fn test_resolve_base_url_anthropic() {
    let cfg = default_config(Provider::Anthropic);
    assert_eq!(llm::resolve_base_url(&cfg), "https://api.anthropic.com/v1");
}

#[test]
fn test_resolve_base_url_custom() {
    let cfg = CompletionConfig {
        provider: Provider::Custom,
        custom: CustomProviderConfig {
            base_url: "http://localhost:1234/v1".to_string(),
            model: "test".to_string(),
        },
        ..default_config(Provider::Custom)
    };
    assert_eq!(llm::resolve_base_url(&cfg), "http://localhost:1234/v1");
}

#[test]
fn test_resolve_model_defaults() {
    assert_eq!(
        llm::resolve_model(&default_config(Provider::OpenAI)),
        "gpt-4o-mini"
    );
    assert_eq!(
        llm::resolve_model(&default_config(Provider::Anthropic)),
        "claude-haiku-3-5"
    );
    assert_eq!(
        llm::resolve_model(&default_config(Provider::Google)),
        "gemini-2.0-flash"
    );
}

#[test]
fn test_resolve_model_custom_override() {
    let mut cfg = default_config(Provider::OpenAI);
    cfg.model = "gpt-4o".to_string();
    assert_eq!(llm::resolve_model(&cfg), "gpt-4o");
}

#[test]
fn test_is_anthropic() {
    assert!(llm::is_anthropic(&Provider::Anthropic));
    assert!(!llm::is_anthropic(&Provider::OpenAI));
    assert!(!llm::is_anthropic(&Provider::Ollama));
}

#[test]
fn test_build_openai_request_body() {
    let body = llm::build_openai_request_body("gpt-4o-mini", "system prompt", "user content");
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["model"], "gpt-4o-mini");
    assert!(parsed["stream"].as_bool().unwrap());
    assert!(parsed["messages"].is_array());
    assert_eq!(parsed["messages"][0]["role"], "system");
    assert_eq!(parsed["messages"][1]["role"], "user");
}

#[test]
fn test_build_anthropic_request_body() {
    let body = llm::build_anthropic_request_body("claude-haiku", "system prompt", "user content");
    let parsed: serde_json::Value = serde_json::from_str(&body).unwrap();
    assert_eq!(parsed["model"], "claude-haiku");
    assert!(parsed["stream"].as_bool().unwrap());
    assert_eq!(parsed["system"], "system prompt");
    assert_eq!(parsed["messages"][0]["role"], "user");
}
