use crate::models::config::{CompletionConfig, Provider};
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

const SYSTEM_PROMPT: &str =
    "You are a markdown writing assistant. Continue the text naturally. Only output the continuation, no explanation.";

/// Resolves the base URL for the given provider configuration.
pub fn resolve_base_url(config: &CompletionConfig) -> String {
    if !config.base_url.is_empty() {
        return config.base_url.clone();
    }

    match config.provider {
        Provider::Ollama => "http://localhost:11434/v1".to_string(),
        Provider::OpenAI => "https://api.openai.com/v1".to_string(),
        Provider::Google => "https://generativelanguage.googleapis.com/v1beta/openai".to_string(),
        Provider::Anthropic => "https://api.anthropic.com/v1".to_string(),
        Provider::Custom => {
            if !config.custom.base_url.is_empty() {
                config.custom.base_url.clone()
            } else {
                "http://localhost:11434/v1".to_string()
            }
        }
    }
}

/// Resolves the model name for the given provider configuration.
pub fn resolve_model(config: &CompletionConfig) -> String {
    if !config.model.is_empty() {
        return config.model.clone();
    }

    match config.provider {
        Provider::Ollama => "llama3.2".to_string(),
        Provider::OpenAI => "gpt-4o-mini".to_string(),
        Provider::Google => "gemini-2.0-flash".to_string(),
        Provider::Anthropic => "claude-3-5-haiku-latest".to_string(),
        Provider::Custom => {
            if !config.custom.model.is_empty() {
                config.custom.model.clone()
            } else {
                "llama3.2".to_string()
            }
        }
    }
}

/// Returns true if the provider is Anthropic (uses a different API format).
pub fn is_anthropic(provider: &Provider) -> bool {
    matches!(provider, Provider::Anthropic)
}

/// Builds a JSON request body for OpenAI-compatible APIs.
pub fn build_openai_request_body(model: &str, system: &str, content: &str) -> String {
    serde_json::json!({
        "model": model,
        "stream": true,
        "messages": [
            { "role": "system", "content": system },
            { "role": "user", "content": content }
        ]
    })
    .to_string()
}

/// Builds a JSON request body for the Anthropic Messages API.
pub fn build_anthropic_request_body(model: &str, system: &str, content: &str) -> String {
    serde_json::json!({
        "model": model,
        "stream": true,
        "max_tokens": 1024,
        "system": system,
        "messages": [
            { "role": "user", "content": content }
        ]
    })
    .to_string()
}

/// Streams a completion from the configured LLM provider.
///
/// Calls `on_chunk` with each text piece as it arrives via SSE.
pub async fn stream_completion(
    config: &CompletionConfig,
    context: &str,
    on_chunk: impl Fn(String),
) -> Result<(), String> {
    let base_url = resolve_base_url(config);
    let model = resolve_model(config);
    let anthropic = is_anthropic(&config.provider);

    let (url, body) = if anthropic {
        let url = format!("{}/messages", base_url);
        let body = build_anthropic_request_body(&model, SYSTEM_PROMPT, context);
        (url, body)
    } else {
        let url = format!("{}/chat/completions", base_url);
        let body = build_openai_request_body(&model, SYSTEM_PROMPT, context);
        (url, body)
    };

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    if anthropic {
        if !config.api_key.is_empty() {
            headers.insert(
                "x-api-key",
                HeaderValue::from_str(&config.api_key).map_err(|e| e.to_string())?,
            );
        }
        headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));
    } else if !config.api_key.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))
                .map_err(|e| e.to_string())?,
        );
    }

    // SEC-005: Validate URL scheme for non-loopback hosts
    if let Ok(parsed_url) = reqwest::Url::parse(&url) {
        let is_loopback = parsed_url.host_str().map_or(false, |h| {
            h == "localhost" || h == "127.0.0.1" || h == "::1"
        });
        if !is_loopback && parsed_url.scheme() != "https" {
            return Err("Non-loopback LLM endpoints must use HTTPS".to_string());
        }
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;
    let response = client
        .post(&url)
        .headers(headers)
        .body(body)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("LLM API error ({}): {}", status, text));
    }

    let mut stream = response.bytes_stream();

    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Stream error: {}", e))?;
        let text = String::from_utf8_lossy(&chunk);
        buffer.push_str(&text);

        // Process complete SSE lines from the buffer.
        // SSE protocol sends lines prefixed with field names: "data:", "event:", "id:", "retry:".
        // We only care about "data:" lines — skip everything else including blank lines.
        while let Some(newline_pos) = buffer.find('\n') {
            let line = buffer[..newline_pos].trim().to_string();
            buffer = buffer[newline_pos + 1..].to_string();

            if line.is_empty()
                || line.starts_with("event:")
                || line.starts_with("id:")
                || line.starts_with("retry:")
                || !line.starts_with("data: ")
            {
                continue;
            }

            let data = &line[6..];
            if data == "[DONE]" {
                return Ok(());
            }

            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) {
                let content_text = if anthropic {
                    // Anthropic format: delta.text
                    parsed
                        .get("delta")
                        .and_then(|d| d.get("text"))
                        .and_then(|t| t.as_str())
                        .map(String::from)
                } else {
                    // OpenAI format: choices[0].delta.content
                    parsed
                        .get("choices")
                        .and_then(|c| c.get(0))
                        .and_then(|c| c.get("delta"))
                        .and_then(|d| d.get("content"))
                        .and_then(|t| t.as_str())
                        .map(String::from)
                };

                if let Some(text) = content_text {
                    if !text.is_empty() {
                        on_chunk(text);
                    }
                }
            }
        }
    }

    Ok(())
}
