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
