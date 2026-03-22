use markdowned_lib::services::renderer;

#[test]
fn test_basic_markdown_to_html() {
    let result = renderer::render_markdown("# Hello\n\nWorld");
    assert!(result.contains("<h1>Hello</h1>"));
    assert!(result.contains("<p>World</p>"));
}

#[test]
fn test_code_block_with_syntax_highlighting() {
    let md = "```rust\nfn main() {}\n```";
    let result = renderer::render_markdown(md);
    assert!(result.contains("<pre"));
    assert!(result.contains("fn"));
}

#[test]
fn test_gfm_table() {
    let md = "| A | B |\n|---|---|\n| 1 | 2 |";
    let result = renderer::render_markdown(md);
    assert!(result.contains("<table>"));
}

#[test]
fn test_gfm_task_list() {
    let md = "- [x] Done\n- [ ] Todo";
    let result = renderer::render_markdown(md);
    assert!(result.contains("checked"));
}

#[test]
fn test_frontmatter_extraction() {
    let md = "---\ntitle: Test\ndate: 2026-03-20\n---\n\n# Content";
    let (frontmatter, html) = renderer::render_markdown_with_frontmatter(md);
    assert!(frontmatter.is_some());
    let fm = frontmatter.unwrap();
    assert!(fm.contains("title"));
    assert!(html.contains("<h1>Content</h1>"));
}

#[test]
fn test_no_frontmatter() {
    let md = "# Just content";
    let (frontmatter, html) = renderer::render_markdown_with_frontmatter(md);
    assert!(frontmatter.is_none());
    assert!(html.contains("<h1>Just content</h1>"));
}

#[test]
fn test_script_tags_stripped() {
    let md = "Hello\n\n<script>alert('xss')</script>\n\nWorld";
    let result = renderer::render_markdown(md);
    assert!(!result.contains("<script>"));
    assert!(!result.contains("alert"));
}

#[test]
fn test_onerror_stripped() {
    let md = "<img src=x onerror=\"alert('xss')\">";
    let result = renderer::render_markdown(md);
    assert!(!result.contains("onerror"));
}

#[test]
fn test_syntax_highlighting_preserved_after_sanitization() {
    let md = "```rust\nfn main() {}\n```";
    let result = renderer::render_markdown(md);
    assert!(result.contains("<pre"));
    assert!(result.contains("style="));
}

#[test]
fn test_remote_image_sources_stripped() {
    let md = "<img src=\"https://example.com/track.png\" alt=\"tracker\">";
    let result = renderer::render_markdown(md);
    assert!(!result.contains("https://example.com/track.png"));
    assert!(!result.contains("src="));
    assert!(result.contains("alt=\"tracker\""));
}

#[test]
fn test_background_urls_stripped_from_styles() {
    let md = "<span style=\"background:url(https://example.com/bg.png);color:red\">x</span>";
    let result = renderer::render_markdown(md);
    assert!(result.contains("color:red"));
    assert!(!result.contains("background"));
    assert!(!result.contains("example.com"));
}
