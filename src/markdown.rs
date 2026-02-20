// Markdown-to-HTML renderer.
//
// Converts a CommonMark subset of Markdown to a complete HTML5 page.
// Single-pass, line-oriented parser with zero external dependencies.

/// Escape HTML special characters in text content.
pub fn escape_html(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Wrap rendered HTML body in a complete HTML5 page.
/// Embedded CSS stylesheet for all HTML pages.
const CSS: &str = r#"
body {
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
    font-size: 16px;
    line-height: 1.6;
    color: #24292e;
    max-width: 48em;
    margin: 0 auto;
    padding: 1em 2em;
}
h1, h2, h3, h4, h5, h6 { margin-top: 1.5em; margin-bottom: 0.5em; font-weight: 600; }
h1 { font-size: 2em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
h2 { font-size: 1.5em; border-bottom: 1px solid #eaecef; padding-bottom: 0.3em; }
h3 { font-size: 1.25em; }
h4 { font-size: 1em; }
h5 { font-size: 0.875em; }
h6 { font-size: 0.85em; color: #6a737d; }
p { margin: 0.5em 0 1em; }
a { color: #0366d6; text-decoration: none; }
a:hover { text-decoration: underline; }
code {
    font-family: "SFMono-Regular", Consolas, "Liberation Mono", Menlo, monospace;
    font-size: 0.9em;
    background: #f0f0f0;
    padding: 0.2em 0.4em;
    border-radius: 3px;
}
pre {
    background: #f6f8fa;
    border: 1px solid #e1e4e8;
    border-radius: 6px;
    padding: 1em;
    overflow-x: auto;
    line-height: 1.45;
}
pre code {
    background: none;
    padding: 0;
    font-size: 0.9em;
}
blockquote {
    border-left: 4px solid #dfe2e5;
    padding: 0.5em 1em;
    margin: 1em 0;
    color: #6a737d;
}
ul, ol { padding-left: 2em; }
li { padding: 0.25em 0; }
hr {
    border: none;
    border-top: 1px solid #e1e4e8;
    margin: 1.5em 0;
}
img { max-width: 100%; }
"#;

pub fn wrap_html_page(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<title>{}</title>\n<style>{}</style>\n</head>\n<body>{}</body>\n</html>\n",
        escape_html(title),
        CSS,
        body
    )
}

/// Extract title from the first `#` heading, falling back to filename.
pub fn extract_title(source: &str, filename: &str) -> String {
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            return rest.trim().to_string();
        }
    }
    filename.strip_suffix(".md").unwrap_or(filename).to_string()
}

/// Process inline formatting: bold, italic, combined, code, links, images.
fn render_inline(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Inline code: `...`
        if chars[i] == '`'
            && let Some(end) = find_char(&chars, '`', i + 1)
        {
            out.push_str("<code>");
            let code_text: String = chars[i + 1..end].iter().collect();
            out.push_str(&escape_html(&code_text));
            out.push_str("</code>");
            i = end + 1;
            continue;
        }

        // Image: ![alt](url)
        if chars[i] == '!'
            && i + 1 < len
            && chars[i + 1] == '['
            && let Some((alt, url, end)) = parse_link_or_image(&chars, i + 1)
        {
            out.push_str(&format!(
                "<img src=\"{}\" alt=\"{}\">",
                escape_html(&url),
                escape_html(&alt)
            ));
            i = end;
            continue;
        }

        // Link: [text](url)
        if chars[i] == '['
            && let Some((text_content, url, end)) = parse_link_or_image(&chars, i)
        {
            out.push_str(&format!(
                "<a href=\"{}\">{}</a>",
                escape_html(&url),
                render_inline(&text_content)
            ));
            i = end;
            continue;
        }

        // Bold+italic: ***text***
        if i + 2 < len
            && chars[i] == '*'
            && chars[i + 1] == '*'
            && chars[i + 2] == '*'
            && let Some(end) = find_sequence(&chars, &['*', '*', '*'], i + 3)
        {
            let inner: String = chars[i + 3..end].iter().collect();
            out.push_str("<strong><em>");
            out.push_str(&render_inline(&inner));
            out.push_str("</em></strong>");
            i = end + 3;
            continue;
        }

        // Bold: **text**
        if i + 1 < len
            && chars[i] == '*'
            && chars[i + 1] == '*'
            && let Some(end) = find_sequence(&chars, &['*', '*'], i + 2)
        {
            let inner: String = chars[i + 2..end].iter().collect();
            out.push_str("<strong>");
            out.push_str(&render_inline(&inner));
            out.push_str("</strong>");
            i = end + 2;
            continue;
        }

        // Italic: *text*
        if chars[i] == '*'
            && let Some(end) = find_char(&chars, '*', i + 1)
        {
            let inner: String = chars[i + 1..end].iter().collect();
            out.push_str("<em>");
            out.push_str(&render_inline(&inner));
            out.push_str("</em>");
            i = end + 1;
            continue;
        }

        // Regular character — escape HTML
        match chars[i] {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(chars[i]),
        }
        i += 1;
    }
    out
}

fn find_char(chars: &[char], target: char, start: usize) -> Option<usize> {
    (start..chars.len()).find(|&i| chars[i] == target)
}

fn find_sequence(chars: &[char], seq: &[char], start: usize) -> Option<usize> {
    let slen = seq.len();
    if chars.len() < slen {
        return None;
    }
    (start..=chars.len() - slen).find(|&i| chars[i..i + slen] == *seq)
}

fn parse_link_or_image(chars: &[char], bracket_start: usize) -> Option<(String, String, usize)> {
    // Find closing ]
    let close_bracket = find_char(chars, ']', bracket_start + 1)?;
    // Must be followed by (
    if close_bracket + 1 >= chars.len() || chars[close_bracket + 1] != '(' {
        return None;
    }
    let url_start = close_bracket + 2;
    let url_end = find_char(chars, ')', url_start)?;
    let text: String = chars[bracket_start + 1..close_bracket].iter().collect();
    let url: String = chars[url_start..url_end].iter().collect();
    Some((text, url, url_end + 1))
}

#[derive(PartialEq)]
enum BlockState {
    None,
    Paragraph,
    FencedCode(String), // language
    IndentedCode,
    Blockquote,
}

#[derive(PartialEq, Clone, Copy)]
enum ListKind {
    Unordered,
    Ordered,
}

struct ListEntry {
    kind: ListKind,
    indent: usize,
}

/// Render Markdown source to a complete HTML5 page.
pub fn render(source: &str, filename: &str) -> String {
    let title = extract_title(source, filename);
    let body = render_body(source);
    wrap_html_page(&title, &body)
}

fn render_body(source: &str) -> String {
    let normalized = source.replace("\r\n", "\n");
    let lines: Vec<&str> = normalized.split('\n').collect();
    let mut out = String::with_capacity(source.len());
    let mut state = BlockState::None;
    let mut para_buf = String::new();
    let mut code_buf = String::new();
    let mut list_stack: Vec<ListEntry> = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim_end();

        // Inside fenced code block
        if let BlockState::FencedCode(_) = &state {
            if trimmed.starts_with("```") {
                out.push_str(&escape_html(&code_buf));
                out.push_str("</code></pre>\n");
                code_buf.clear();
                state = BlockState::None;
            } else {
                if !code_buf.is_empty() {
                    code_buf.push('\n');
                }
                code_buf.push_str(line);
            }
            i += 1;
            continue;
        }

        // Inside indented code block
        if state == BlockState::IndentedCode {
            if line.starts_with("    ") || (trimmed.is_empty() && peek_indented_code(&lines, i)) {
                if !code_buf.is_empty() {
                    code_buf.push('\n');
                }
                if let Some(stripped) = line.strip_prefix("    ") {
                    code_buf.push_str(stripped);
                }
                i += 1;
                continue;
            } else {
                out.push_str(&escape_html(&code_buf));
                out.push_str("</code></pre>\n");
                code_buf.clear();
                state = BlockState::None;
                // fall through to process this line
            }
        }

        // Blank line
        if trimmed.is_empty() {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            if state == BlockState::Blockquote {
                out.push_str("</blockquote>\n");
                state = BlockState::None;
            }
            i += 1;
            continue;
        }

        // Fenced code block start
        if let Some(rest) = trimmed.strip_prefix("```") {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            let lang = rest.trim().to_string();
            if lang.is_empty() {
                out.push_str("<pre><code>");
            } else {
                out.push_str(&format!(
                    "<pre><code class=\"language-{}\">",
                    escape_html(&lang)
                ));
            }
            state = BlockState::FencedCode(lang);
            i += 1;
            continue;
        }

        // Heading
        if let Some((level, text)) = parse_heading(trimmed) {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            out.push_str(&format!(
                "<h{}>{}</h{}>\n",
                level,
                render_inline(text),
                level
            ));
            i += 1;
            continue;
        }

        // Horizontal rule
        if is_horizontal_rule(trimmed) {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            out.push_str("<hr>\n");
            i += 1;
            continue;
        }

        // Blockquote
        if let Some(rest) = trimmed
            .strip_prefix("> ")
            .or_else(|| if trimmed == ">" { Some("") } else { None })
        {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            if state != BlockState::Blockquote {
                out.push_str("<blockquote>\n");
                state = BlockState::Blockquote;
            }
            out.push_str(&format!("<p>{}</p>\n", render_inline(rest)));
            i += 1;
            continue;
        }

        // Unordered list item
        if let Some((indent, text)) = parse_unordered_list_item(line) {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            handle_list_item(&mut out, &mut list_stack, ListKind::Unordered, indent, text);
            i += 1;
            continue;
        }

        // Ordered list item
        if let Some((indent, text)) = parse_ordered_list_item(line) {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            handle_list_item(&mut out, &mut list_stack, ListKind::Ordered, indent, text);
            i += 1;
            continue;
        }

        // Indented code block (4 spaces, not inside a list)
        if line.starts_with("    ") && list_stack.is_empty() && state == BlockState::None {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            out.push_str("<pre><code>");
            code_buf.push_str(&line[4..]);
            state = BlockState::IndentedCode;
            i += 1;
            continue;
        }

        // Paragraph text
        close_all_lists(&mut out, &mut list_stack);
        if state == BlockState::Blockquote {
            out.push_str("</blockquote>\n");
            state = BlockState::None;
        }
        if state != BlockState::Paragraph {
            state = BlockState::Paragraph;
        }
        if !para_buf.is_empty() {
            para_buf.push('\n');
        }
        para_buf.push_str(trimmed);
        i += 1;
    }

    // Flush remaining state
    flush_paragraph(&mut out, &mut para_buf, &mut state);
    close_all_lists(&mut out, &mut list_stack);
    if state == BlockState::Blockquote {
        out.push_str("</blockquote>\n");
    }
    if let BlockState::FencedCode(_) = &state {
        out.push_str(&escape_html(&code_buf));
        out.push_str("</code></pre>\n");
    }
    if state == BlockState::IndentedCode {
        out.push_str(&escape_html(&code_buf));
        out.push_str("</code></pre>\n");
    }

    out
}

fn flush_paragraph(out: &mut String, para_buf: &mut String, state: &mut BlockState) {
    if *state == BlockState::Paragraph && !para_buf.is_empty() {
        out.push_str(&format!("<p>{}</p>\n", render_inline(para_buf)));
        para_buf.clear();
    }
    if *state == BlockState::Paragraph {
        *state = BlockState::None;
    }
}

fn parse_heading(line: &str) -> Option<(u8, &str)> {
    let bytes = line.as_bytes();
    let mut level = 0u8;
    while (level as usize) < bytes.len() && bytes[level as usize] == b'#' {
        level += 1;
    }
    if level == 0 || level > 6 {
        return None;
    }
    if (level as usize) < bytes.len() && bytes[level as usize] == b' ' {
        Some((level, &line[level as usize + 1..]))
    } else {
        None
    }
}

fn is_horizontal_rule(line: &str) -> bool {
    let stripped: String = line.chars().filter(|c| !c.is_whitespace()).collect();
    if stripped.len() < 3 {
        return false;
    }
    let first = stripped.chars().next().unwrap();
    (first == '-' || first == '*' || first == '_') && stripped.chars().all(|c| c == first)
}

fn parse_unordered_list_item(line: &str) -> Option<(usize, &str)> {
    let indent = line.len() - line.trim_start().len();
    let trimmed = line.trim_start();
    if let Some(rest) = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .or_else(|| trimmed.strip_prefix("+ "))
    {
        Some((indent, rest))
    } else {
        None
    }
}

fn parse_ordered_list_item(line: &str) -> Option<(usize, &str)> {
    let indent = line.len() - line.trim_start().len();
    let trimmed = line.trim_start();
    let mut num_end = 0;
    let bytes = trimmed.as_bytes();
    while num_end < bytes.len() && bytes[num_end].is_ascii_digit() {
        num_end += 1;
    }
    if num_end == 0 || num_end >= bytes.len() {
        return None;
    }
    if bytes[num_end] == b'.' && num_end + 1 < bytes.len() && bytes[num_end + 1] == b' ' {
        Some((indent, &trimmed[num_end + 2..]))
    } else {
        None
    }
}

fn handle_list_item(
    out: &mut String,
    stack: &mut Vec<ListEntry>,
    kind: ListKind,
    indent: usize,
    text: &str,
) {
    // Close deeper lists
    while stack.len() > 1 {
        let last = stack.last().unwrap();
        if indent < last.indent {
            close_list_top(out, stack);
        } else {
            break;
        }
    }

    if stack.is_empty() || indent > stack.last().unwrap().indent {
        // Open new list level
        let tag = if kind == ListKind::Unordered {
            "<ul>\n"
        } else {
            "<ol>\n"
        };
        out.push_str(tag);
        stack.push(ListEntry { kind, indent });
    }

    out.push_str(&format!("<li>{}</li>\n", render_inline(text)));
}

fn close_list_top(out: &mut String, stack: &mut Vec<ListEntry>) {
    if let Some(entry) = stack.pop() {
        let tag = if entry.kind == ListKind::Unordered {
            "</ul>\n"
        } else {
            "</ol>\n"
        };
        out.push_str(tag);
    }
}

fn close_all_lists(out: &mut String, stack: &mut Vec<ListEntry>) {
    while !stack.is_empty() {
        close_list_top(out, stack);
    }
}

fn peek_indented_code(lines: &[&str], blank_idx: usize) -> bool {
    for l in &lines[blank_idx + 1..] {
        let l = l.trim_end();
        if l.is_empty() {
            continue;
        }
        return l.starts_with("    ");
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Foundational tests ===

    #[test]
    fn test_escape_html_angle_brackets() {
        assert_eq!(escape_html("<div>"), "&lt;div&gt;");
    }

    #[test]
    fn test_escape_html_ampersand() {
        assert_eq!(escape_html("a & b"), "a &amp; b");
    }

    #[test]
    fn test_escape_html_quote() {
        assert_eq!(escape_html(r#"say "hello""#), "say &quot;hello&quot;");
    }

    #[test]
    fn test_escape_html_mixed() {
        assert_eq!(escape_html("<a>&\""), "&lt;a&gt;&amp;&quot;");
    }

    #[test]
    fn test_escape_html_no_escaping_needed() {
        assert_eq!(escape_html("hello world"), "hello world");
    }

    #[test]
    fn test_wrap_html_page() {
        let page = wrap_html_page("My Title", "<p>hello</p>");
        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains("<title>My Title</title>"));
        assert!(page.contains("<p>hello</p>"));
        assert!(page.contains("<meta charset=\"utf-8\">"));
        assert!(page.contains("</html>"));
        // Styling additions
        assert!(page.contains("<style>"));
        assert!(page.contains("</style>"));
        assert!(
            page.contains(
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
            )
        );
    }

    #[test]
    fn test_wrap_html_page_empty_body() {
        let page = wrap_html_page("Empty", "");
        assert!(page.contains("<body></body>"));
        assert!(page.contains("<style>"));
    }

    #[test]
    fn test_wrap_html_page_contains_style_block() {
        let page = wrap_html_page("Test", "<p>hi</p>");
        assert!(page.contains("<style>"));
        assert!(page.contains("</style>"));
    }

    #[test]
    fn test_wrap_html_page_contains_viewport_meta() {
        let page = wrap_html_page("Test", "");
        assert!(page.contains("name=\"viewport\""));
        assert!(page.contains("width=device-width"));
    }

    #[test]
    fn test_wrap_html_page_no_external_css() {
        let page = wrap_html_page("Test", "");
        assert!(!page.contains("<link"));
        assert!(!page.contains("@import"));
    }

    #[test]
    fn test_css_contains_typography_rules() {
        let page = wrap_html_page("Test", "");
        assert!(page.contains("font-family:"));
        assert!(page.contains("max-width:"));
        assert!(page.contains("line-height:"));
    }

    #[test]
    fn test_css_contains_element_rules() {
        let page = wrap_html_page("Test", "");
        // Headings
        assert!(page.contains("h1"));
        assert!(page.contains("h6"));
        // Code
        assert!(page.contains("pre {"));
        assert!(page.contains("code {"));
        assert!(page.contains("overflow-x: auto"));
        // Links
        assert!(page.contains("a {"));
        assert!(page.contains("#0366d6"));
        // Blockquotes
        assert!(page.contains("blockquote"));
        assert!(page.contains("#dfe2e5"));
        // HR
        assert!(page.contains("hr {"));
        // Images
        assert!(page.contains("img {"));
    }

    #[test]
    fn test_extract_title_from_heading() {
        assert_eq!(
            extract_title("# Hello World\nsome text", "file.md"),
            "Hello World"
        );
    }

    #[test]
    fn test_extract_title_fallback_to_filename() {
        assert_eq!(extract_title("no heading here", "notes.md"), "notes");
    }

    #[test]
    fn test_extract_title_strips_md_extension() {
        assert_eq!(extract_title("just text", "README.md"), "README");
    }

    // === US1: Heading rendering ===

    #[test]
    fn test_heading_h1() {
        let body = render_body("# Hello");
        assert_eq!(body.trim(), "<h1>Hello</h1>");
    }

    #[test]
    fn test_heading_with_angle_brackets() {
        let body = render_body("# rd::expected<void, E>");
        assert!(
            body.contains("<h1>rd::expected&lt;void, E&gt;</h1>"),
            "got: {}",
            body
        );
        // Must NOT double-escape to &amp;lt;
        assert!(!body.contains("&amp;"), "double-escaped: {}", body);
    }

    #[test]
    fn test_render_no_double_escape() {
        let page = render(
            "# rd::expected<void, E>\n\nSome text with <html> & \"quotes\"",
            "test.md",
        );
        // Title should be escaped once
        assert!(
            page.contains("<title>rd::expected&lt;void, E&gt;</title>"),
            "title: {}",
            page
        );
        // Body heading should be escaped once
        assert!(
            page.contains("<h1>rd::expected&lt;void, E&gt;</h1>"),
            "heading: {}",
            page
        );
        // Paragraph should be escaped once
        assert!(page.contains("&lt;html&gt;"), "html in para: {}", page);
        assert!(page.contains("&amp;"), "amp in para: {}", page);
        // Must NOT have &amp;lt; anywhere (double escape)
        assert!(!page.contains("&amp;lt;"), "double-escaped: {}", page);
        assert!(!page.contains("&amp;gt;"), "double-escaped: {}", page);
        assert!(!page.contains("&amp;amp;"), "double-escaped: {}", page);
    }

    #[test]
    fn test_heading_h2_through_h6() {
        assert!(render_body("## Sub").contains("<h2>Sub</h2>"));
        assert!(render_body("### Sub").contains("<h3>Sub</h3>"));
        assert!(render_body("#### Sub").contains("<h4>Sub</h4>"));
        assert!(render_body("##### Sub").contains("<h5>Sub</h5>"));
        assert!(render_body("###### Sub").contains("<h6>Sub</h6>"));
    }

    // === US1: Paragraph rendering ===

    #[test]
    fn test_paragraph_single() {
        let body = render_body("Hello world");
        assert_eq!(body.trim(), "<p>Hello world</p>");
    }

    #[test]
    fn test_paragraph_separated_by_blank_line() {
        let body = render_body("First para\n\nSecond para");
        assert!(body.contains("<p>First para</p>"));
        assert!(body.contains("<p>Second para</p>"));
    }

    // === US1: Inline formatting ===

    #[test]
    fn test_inline_bold() {
        let body = render_body("**bold text**");
        assert!(body.contains("<strong>bold text</strong>"));
    }

    #[test]
    fn test_inline_italic() {
        let body = render_body("*italic text*");
        assert!(body.contains("<em>italic text</em>"));
    }

    #[test]
    fn test_inline_combined_bold_italic() {
        let body = render_body("***both***");
        assert!(body.contains("<strong><em>both</em></strong>"));
    }

    #[test]
    fn test_inline_code() {
        let body = render_body("`some code`");
        assert!(body.contains("<code>some code</code>"));
    }

    #[test]
    fn test_inline_code_escapes_html() {
        let body = render_body("`<div>`");
        assert!(body.contains("<code>&lt;div&gt;</code>"));
    }

    // === US1: Link rendering ===

    #[test]
    fn test_link() {
        let body = render_body("[click here](https://example.com)");
        assert!(body.contains("<a href=\"https://example.com\">click here</a>"));
    }

    // === US1: Fenced code block ===

    #[test]
    fn test_fenced_code_block_with_language() {
        let body = render_body("```rust\nfn main() {}\n```");
        assert!(body.contains("<pre><code class=\"language-rust\">"));
        assert!(body.contains("fn main() {}"));
        assert!(body.contains("</code></pre>"));
    }

    #[test]
    fn test_fenced_code_block_no_language() {
        let body = render_body("```\nhello\n```");
        assert!(body.contains("<pre><code>hello</code></pre>"));
    }

    #[test]
    fn test_fenced_code_block_escapes_html() {
        let body = render_body("```\n<div>&</div>\n```");
        assert!(body.contains("&lt;div&gt;&amp;&lt;/div&gt;"));
    }

    // === US1: Indented code block ===

    #[test]
    fn test_indented_code_block() {
        let body = render_body("    let x = 1;\n    let y = 2;");
        assert!(body.contains("<pre><code>let x = 1;\nlet y = 2;</code></pre>"));
    }

    // === US1: Full render ===

    #[test]
    fn test_render_produces_html_page() {
        let page = render("# Test\n\nHello world", "test.md");
        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains("<title>Test</title>"));
        assert!(page.contains("<h1>Test</h1>"));
        assert!(page.contains("<p>Hello world</p>"));
    }

    // === US3: Unordered lists ===

    #[test]
    fn test_unordered_list() {
        let body = render_body("- item one\n- item two");
        assert!(body.contains("<ul>"));
        assert!(body.contains("<li>item one</li>"));
        assert!(body.contains("<li>item two</li>"));
        assert!(body.contains("</ul>"));
    }

    // === US3: Ordered lists ===

    #[test]
    fn test_ordered_list() {
        let body = render_body("1. first\n2. second");
        assert!(body.contains("<ol>"));
        assert!(body.contains("<li>first</li>"));
        assert!(body.contains("<li>second</li>"));
        assert!(body.contains("</ol>"));
    }

    // === US3: Nested lists ===

    #[test]
    fn test_nested_list() {
        let body = render_body("- outer\n  - inner\n    - deep");
        assert!(body.contains("<ul>"));
        assert!(body.contains("<li>outer</li>"));
        assert!(body.contains("<li>inner</li>"));
        assert!(body.contains("<li>deep</li>"));
        // Should have nested ul elements
        let ul_count = body.matches("<ul>").count();
        assert!(ul_count >= 2, "expected nested <ul>, got body: {}", body);
    }

    // === US3: Blockquote ===

    #[test]
    fn test_blockquote() {
        let body = render_body("> quoted text");
        assert!(body.contains("<blockquote>"));
        assert!(body.contains("<p>quoted text</p>"));
        assert!(body.contains("</blockquote>"));
    }

    // === US3: Horizontal rule ===

    #[test]
    fn test_horizontal_rule_dashes() {
        let body = render_body("---");
        assert!(body.contains("<hr>"));
    }

    #[test]
    fn test_horizontal_rule_asterisks() {
        let body = render_body("***");
        // *** could be hr or empty bold+italic; we treat standalone line as hr
        assert!(body.contains("<hr>"));
    }

    #[test]
    fn test_horizontal_rule_underscores() {
        let body = render_body("___");
        assert!(body.contains("<hr>"));
    }

    // === US4: Images ===

    #[test]
    fn test_image() {
        let body = render_body("![alt text](image.png)");
        assert!(body.contains("<img src=\"image.png\" alt=\"alt text\">"));
    }

    #[test]
    fn test_image_escapes_alt_text() {
        let body = render_body("![a <b> & c](pic.jpg)");
        assert!(body.contains("alt=\"a &lt;b&gt; &amp; c\""));
    }

    // === Edge cases ===

    #[test]
    fn test_empty_markdown() {
        let page = render("", "empty.md");
        assert!(page.contains("<title>empty</title>"));
        assert!(page.contains("<body>"));
    }

    #[test]
    fn test_crlf_line_endings() {
        let body_lf = render_body("# Hello\n\nWorld");
        let body_crlf = render_body("# Hello\r\n\r\nWorld");
        assert_eq!(body_lf, body_crlf);
    }

    #[test]
    fn test_html_escaping_in_heading() {
        let body = render_body("# Hello <world> & \"friends\"");
        assert!(body.contains("<h1>Hello &lt;world&gt; &amp; &quot;friends&quot;</h1>"));
    }

    #[test]
    fn test_html_escaping_in_list_item() {
        let body = render_body("- item with <html> & stuff");
        assert!(body.contains("<li>item with &lt;html&gt; &amp; stuff</li>"));
    }
}
