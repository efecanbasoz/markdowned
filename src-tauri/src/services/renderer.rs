use std::sync::LazyLock;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

static SYNTAX_SET: LazyLock<SyntaxSet> = LazyLock::new(SyntaxSet::load_defaults_newlines);
static THEME_SET: LazyLock<ThemeSet> = LazyLock::new(ThemeSet::load_defaults);

/// Extract YAML frontmatter from the beginning of markdown content.
///
/// Returns `(Option<frontmatter_string>, remaining_content)`.
fn extract_frontmatter(input: &str) -> (Option<String>, &str) {
    let trimmed = input.trim_start();
    if !trimmed.starts_with("---") {
        return (None, input);
    }

    // Find the opening delimiter line end
    let after_opening = match trimmed[3..].find('\n') {
        Some(pos) => {
            // Ensure nothing but whitespace after the opening ---
            let rest_of_line = &trimmed[3..3 + pos];
            if !rest_of_line.trim().is_empty() {
                return (None, input);
            }
            3 + pos + 1
        }
        None => return (None, input),
    };

    // Find the closing ---
    if let Some(end_pos) = trimmed[after_opening..].find("\n---") {
        let frontmatter = trimmed[after_opening..after_opening + end_pos].to_string();
        let closing_line_end = after_opening + end_pos + 4; // "\n---" = 4 chars
        // Skip past the closing delimiter line
        let remaining_start = match trimmed[closing_line_end..].find('\n') {
            Some(pos) => closing_line_end + pos + 1,
            None => trimmed.len(),
        };
        // Calculate offset from original input
        let offset = input.len() - trimmed.len();
        let remaining = &input[(offset + remaining_start).min(input.len())..];
        (Some(frontmatter), remaining)
    } else {
        (None, input)
    }
}

/// Highlight a code block using syntect.
///
/// Returns HTML with syntax highlighting, wrapped in `<pre>` and `<code>` tags.
fn highlight_code(code: &str, lang: &str) -> String {
    let syntax = if lang.is_empty() {
        SYNTAX_SET.find_syntax_plain_text()
    } else {
        SYNTAX_SET
            .find_syntax_by_token(lang)
            .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text())
    };

    let theme = &THEME_SET.themes["base16-ocean.dark"];

    match highlighted_html_for_string(code, &SYNTAX_SET, syntax, theme) {
        Ok(highlighted) => {
            // syntect wraps output in <pre style="..."><code>...</code></pre> already,
            // but we want to add our own data-lang attribute. The output starts with
            // <pre style="...">. We inject our class and data-lang into that tag.
            if let Some(rest) = highlighted.strip_prefix("<pre ") {
                format!(
                    "<pre class=\"code-block\" data-lang=\"{}\" {}",
                    html_escape(lang),
                    rest
                )
            } else {
                // Fallback: wrap it ourselves
                format!(
                    "<pre class=\"code-block\" data-lang=\"{}\"><code>{}</code></pre>",
                    html_escape(lang),
                    highlighted
                )
            }
        }
        Err(_) => {
            // Fallback: render as plain code block
            format!(
                "<pre class=\"code-block\" data-lang=\"{}\"><code>{}</code></pre>",
                html_escape(lang),
                html_escape(code)
            )
        }
    }
}

/// Simple HTML entity escaping for attribute values.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Render markdown to HTML.
///
/// Supports GFM extensions: tables, task lists, and strikethrough.
/// Code blocks are syntax-highlighted using syntect.
pub fn render_markdown(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);

    let mut in_code_block = false;
    let mut code_text = String::new();
    let mut code_lang = String::new();

    let events: Vec<Event> = parser
        .flat_map(|event| match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_text.clear();
                code_lang = match &kind {
                    CodeBlockKind::Fenced(lang) => lang.to_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                vec![]
            }
            Event::End(TagEnd::CodeBlock) => {
                in_code_block = false;
                let highlighted = highlight_code(&code_text, &code_lang);
                vec![Event::Html(CowStr::from(highlighted))]
            }
            Event::Text(text) if in_code_block => {
                code_text.push_str(&text);
                vec![]
            }
            other => vec![other],
        })
        .collect();

    let mut html_output = String::new();
    html::push_html(&mut html_output, events.into_iter());
    sanitize_html(&html_output)
}

/// Sanitize HTML output to prevent XSS attacks.
///
/// Allows tags and attributes needed for rendered markdown (tables, code blocks,
/// task lists, etc.) while stripping dangerous elements like `<script>` and
/// event handler attributes.
/// SEC-007: Filter CSS to prevent UI redressing attacks.
/// Only allow safe properties needed for syntax highlighting.
fn filter_style(style: &str) -> Option<String> {
    let safe_props = ["color", "background-color", "background", "font-weight",
                      "font-style", "text-decoration", "opacity"];
    let filtered: Vec<&str> = style.split(';')
        .filter(|decl| {
            let trimmed = decl.trim().to_lowercase();
            safe_props.iter().any(|p| trimmed.starts_with(p))
        })
        .collect();
    if filtered.is_empty() { None } else { Some(filtered.join(";")) }
}

fn sanitize_html(html: &str) -> String {
    let cleaned = ammonia::Builder::default()
        .add_tags(&["pre", "code", "span", "table", "thead", "tbody", "tr", "th", "td", "input", "h1", "h2", "h3", "h4", "h5", "h6", "p", "a", "strong", "em", "del", "ul", "ol", "li", "blockquote", "hr", "br", "img"])
        .add_tag_attributes("pre", &["class", "data-lang", "style"])
        .add_tag_attributes("code", &["class", "style"])
        .add_tag_attributes("span", &["style"])
        .add_tag_attributes("input", &["type", "checked", "disabled"])
        .add_tag_attributes("a", &["href", "title"])
        .add_tag_attributes("img", &["src", "alt", "title"])
        .link_rel(Some("noopener noreferrer"))
        .attribute_filter(|_element, attribute, value| {
            if attribute == "style" {
                match filter_style(value) {
                    Some(filtered) => Some(filtered.into()),
                    None => None,
                }
            } else {
                Some(value.into())
            }
        })
        .clean(html)
        .to_string();
    cleaned
}

/// Render markdown to HTML, extracting YAML frontmatter if present.
///
/// Returns `(frontmatter, html)` where frontmatter is `Some(yaml_string)`
/// if the document starts with `---` delimited YAML.
pub fn render_markdown_with_frontmatter(input: &str) -> (Option<String>, String) {
    let (frontmatter, content) = extract_frontmatter(input);
    let html = render_markdown(content);
    (frontmatter, html)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_frontmatter_present() {
        let input = "---\ntitle: Test\n---\n\n# Hello";
        let (fm, rest) = extract_frontmatter(input);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("title: Test"));
        assert!(rest.contains("# Hello"));
    }

    #[test]
    fn test_extract_frontmatter_absent() {
        let input = "# Just content";
        let (fm, rest) = extract_frontmatter(input);
        assert!(fm.is_none());
        assert_eq!(rest, input);
    }

    #[test]
    fn test_html_escape() {
        assert_eq!(html_escape("<div>"), "&lt;div&gt;");
        assert_eq!(html_escape("a&b"), "a&amp;b");
    }
}
