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
/// Embedded CSS stylesheet (main + highlight theme), gzipped.
pub const CSS_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/combined.css.gz"));

/// highlight.js common bundle, gzipped.
pub const HLJS_JS_GZ: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/highlight.min.js.gz"));

pub fn wrap_html_page(title: &str, body: &str, has_custom_css: bool) -> String {
    let css2_link = if has_custom_css {
        "\n<link rel=\"stylesheet\" href=\"/?css2\">"
    } else {
        ""
    };
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n<link rel=\"icon\" href=\"data:image/svg+xml,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 16 16'><text x='0' y='13' font-size='12' font-family='sans-serif' font-weight='bold'>M↓</text></svg>\">\n<title>{}</title>\n<link rel=\"stylesheet\" href=\"/?css\">{}\n</head>\n<body>\n<div class=\"content\">{}</div>\n<script src=\"/?js\"></script>\n<script>\nhljs.highlightAll();\ndocument.querySelectorAll('pre').forEach(function(p){{var b=document.createElement('button');b.className='copy-btn';b.textContent='\u{1F4CB}';b.onclick=function(){{navigator.clipboard.writeText(p.querySelector('code').textContent).then(function(){{b.textContent='\u{2713}';setTimeout(function(){{b.textContent='\u{1F4CB}'}},1500)}});}};p.appendChild(b)}});\n(function(){{var hs=document.querySelectorAll('.content h1,.content h2,.content h3,.content h4,.content h5,.content h6');if(hs.length<2)return;var nav=document.createElement('nav');nav.className='toc';var ul=document.createElement('ul');hs.forEach(function(h){{var li=document.createElement('li');li.className='toc-h'+h.tagName[1];var a=document.createElement('a');a.href='#'+h.id;a.textContent=h.textContent.replace('\u{1F517}','').trim();li.appendChild(a);ul.appendChild(li)}});nav.appendChild(ul);document.body.insertBefore(nav,document.body.firstChild)}})();\n</script>\n</body>\n</html>\n",
        escape_html(title),
        css2_link,
        body,
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
        // Inline code: `...` or ``...`` (multi-backtick spans)
        if chars[i] == '`' {
            let mut ticks = 0;
            while i + ticks < len && chars[i + ticks] == '`' {
                ticks += 1;
            }
            let start = i + ticks;
            // Find matching closing backtick sequence of same length
            let mut j = start;
            let mut found = false;
            while j <= len - ticks {
                if chars[j..j + ticks].iter().all(|&c| c == '`') {
                    found = true;
                    break;
                }
                j += 1;
            }
            if found {
                let mut code_text: String = chars[start..j].iter().collect();
                // Strip one leading/trailing space if both present (GFM rule)
                if code_text.starts_with(' ') && code_text.ends_with(' ') && code_text.len() > 1 {
                    code_text = code_text[1..code_text.len() - 1].to_string();
                }
                out.push_str("<code>");
                out.push_str(&escape_html(&code_text));
                out.push_str("</code>");
                i = j + ticks;
                continue;
            }
            // No closing sequence found — emit backticks as literal text
            for _ in 0..ticks {
                out.push('`');
            }
            i += ticks;
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

        // Strikethrough: ~~text~~
        if i + 1 < len
            && chars[i] == '~'
            && chars[i + 1] == '~'
            && let Some(end) = find_sequence(&chars, &['~', '~'], i + 2)
        {
            let inner: String = chars[i + 2..end].iter().collect();
            out.push_str("<del>");
            out.push_str(&render_inline(&inner));
            out.push_str("</del>");
            i = end + 2;
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

        // Auto-link: bare URLs (https:// or http://)
        if chars[i] == 'h'
            && let Some(url) = try_parse_url(&chars, i)
        {
            out.push_str(&format!(
                "<a href=\"{}\">{}</a>",
                escape_html(&url),
                escape_html(&url)
            ));
            i += url.len();
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

/// Try to parse a bare URL starting at position `start`.
fn try_parse_url(chars: &[char], start: usize) -> Option<String> {
    let rest: String = chars[start..].iter().collect();
    let prefix = if rest.starts_with("https://") {
        "https://"
    } else if rest.starts_with("http://") {
        "http://"
    } else {
        return None;
    };
    // URL ends at whitespace, or certain trailing punctuation
    let after_prefix = &rest[prefix.len()..];
    if after_prefix.is_empty() {
        return None;
    }
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '<' || c == '>' || c == '"')
        .unwrap_or(rest.len());
    // Strip trailing punctuation that's likely not part of the URL
    let url = rest[..end].trim_end_matches(|c: char| ".,;:!?)".contains(c));
    if url.len() <= prefix.len() {
        return None;
    }
    Some(url.to_string())
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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Alignment {
    Left,
    Center,
    Right,
}

fn is_table_separator(line: &str) -> bool {
    let trimmed = line.trim();
    let cells = split_table_cells(trimmed);
    if cells.is_empty() {
        return false;
    }
    cells.iter().all(|cell| {
        let c = cell.trim();
        if c.is_empty() {
            return false;
        }
        let c = c.strip_prefix(':').unwrap_or(c);
        let c = c.strip_suffix(':').unwrap_or(c);
        !c.is_empty() && c.chars().all(|ch| ch == '-')
    })
}

fn parse_alignment(separator: &str) -> Vec<Alignment> {
    split_table_cells(separator)
        .iter()
        .map(|cell| {
            let c = cell.trim();
            let left = c.starts_with(':');
            let right = c.ends_with(':');
            match (left, right) {
                (true, true) => Alignment::Center,
                (false, true) => Alignment::Right,
                _ => Alignment::Left,
            }
        })
        .collect()
}

fn split_table_cells(line: &str) -> Vec<&str> {
    let trimmed = line.trim();
    // Strip leading/trailing pipes
    let inner = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let inner = inner.strip_suffix('|').unwrap_or(inner);

    let mut cells = Vec::new();
    let mut start = 0;
    let bytes = inner.as_bytes();
    let mut in_code = false;
    let mut idx = 0;

    while idx < bytes.len() {
        if bytes[idx] == b'`' {
            in_code = !in_code;
        } else if bytes[idx] == b'|' && !in_code {
            cells.push(inner[start..idx].trim());
            start = idx + 1;
        }
        idx += 1;
    }
    cells.push(inner[start..].trim());
    cells
}

fn render_table(header: &str, separator: &str, data_rows: &[&str]) -> String {
    let alignments = parse_alignment(separator);
    let headers = split_table_cells(header);
    let col_count = headers.len();

    let mut out = String::new();
    out.push_str("<table>\n<thead>\n<tr>\n");

    for (i, h) in headers.iter().enumerate() {
        let align = alignments.get(i).copied().unwrap_or(Alignment::Left);
        let style = alignment_style(align);
        out.push_str(&format!("<th{}>{}</th>\n", style, render_inline(h)));
    }
    out.push_str("</tr>\n</thead>\n<tbody>\n");

    for row in data_rows {
        let cells = split_table_cells(row);
        out.push_str("<tr>\n");
        for i in 0..col_count {
            let align = alignments.get(i).copied().unwrap_or(Alignment::Left);
            let style = alignment_style(align);
            let content = if i < cells.len() { cells[i] } else { "" };
            out.push_str(&format!("<td{}>{}</td>\n", style, render_inline(content)));
        }
        out.push_str("</tr>\n");
    }

    out.push_str("</tbody>\n</table>\n");
    out
}

fn alignment_style(align: Alignment) -> String {
    match align {
        Alignment::Left => String::new(),
        Alignment::Center => " style=\"text-align: center\"".to_string(),
        Alignment::Right => " style=\"text-align: right\"".to_string(),
    }
}

/// Render Markdown source to a complete HTML5 page.
pub fn render(source: &str, filename: &str, has_custom_css: bool) -> String {
    let title = extract_title(source, filename);
    let body = render_body(source);
    wrap_html_page(&title, &body, has_custom_css)
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
            let slug = heading_slug(text);
            out.push_str(&format!(
                "<h{} id=\"{}\"><a class=\"anchor\" href=\"#{}\">\u{1F517}</a>{}</h{}>\n",
                level,
                slug,
                slug,
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

        // Table: current line has pipe, next line is separator
        if trimmed.contains('|') && i + 1 < lines.len() && is_table_separator(lines[i + 1]) {
            flush_paragraph(&mut out, &mut para_buf, &mut state);
            close_all_lists(&mut out, &mut list_stack);
            let header = trimmed;
            let separator = lines[i + 1].trim();
            let mut data_rows: Vec<&str> = Vec::new();
            let mut j = i + 2;
            while j < lines.len() {
                let row = lines[j].trim();
                if row.is_empty() || !row.contains('|') {
                    break;
                }
                data_rows.push(row);
                j += 1;
            }
            out.push_str(&render_table(header, separator, &data_rows));
            i = j;
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

/// Generate a GFM-style anchor slug from heading text.
fn heading_slug(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c.to_ascii_lowercase()
            } else if c == ' ' || c == '-' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect()
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

    // Task list items: - [ ] or - [x]
    if let Some(rest) = text.strip_prefix("[ ] ") {
        out.push_str(&format!(
            "<li><input type=\"checkbox\" disabled> {}</li>\n",
            render_inline(rest)
        ));
    } else if let Some(rest) = text
        .strip_prefix("[x] ")
        .or_else(|| text.strip_prefix("[X] "))
    {
        out.push_str(&format!(
            "<li><input type=\"checkbox\" checked disabled> {}</li>\n",
            render_inline(rest)
        ));
    } else {
        out.push_str(&format!("<li>{}</li>\n", render_inline(text)));
    }
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
        let page = wrap_html_page("My Title", "<p>hello</p>", false);
        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains("<title>My Title</title>"));
        assert!(page.contains("<p>hello</p>"));
        assert!(page.contains("<meta charset=\"utf-8\">"));
        assert!(page.contains("</html>"));
        // Asset link
        assert!(page.contains("<link rel=\"stylesheet\" href=\"/?css\">"));
        assert!(
            page.contains(
                "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">"
            )
        );
    }

    #[test]
    fn test_wrap_html_page_empty_body() {
        let page = wrap_html_page("Empty", "", false);
        assert!(page.contains("<div class=\"content\">"));
        assert!(page.contains("<link"));
    }

    #[test]
    fn test_wrap_html_page_contains_link_tag() {
        let page = wrap_html_page("Test", "<p>hi</p>", false);
        assert!(page.contains("<link"));
        assert!(page.contains("/?css"));
    }

    #[test]
    fn test_wrap_html_page_contains_viewport_meta() {
        let page = wrap_html_page("Test", "", false);
        assert!(page.contains("width=device-width"));
    }

    #[test]
    fn test_wrap_html_page_uses_link_not_inline() {
        let page = wrap_html_page("Test", "", false);
        assert!(page.contains("<link"));
        assert!(!page.contains("<style>"));
        assert!(!page.contains("@import"));
    }

    #[test]
    fn test_wrap_html_page_with_custom_css_includes_css2_link() {
        let page = wrap_html_page("Test", "<p>hi</p>", true);
        assert!(page.contains("<link rel=\"stylesheet\" href=\"/?css\">"));
        assert!(page.contains("<link rel=\"stylesheet\" href=\"/?css2\">"));
    }

    #[test]
    fn test_wrap_html_page_without_custom_css_no_css2_link() {
        let page = wrap_html_page("Test", "<p>hi</p>", false);
        assert!(page.contains("<link rel=\"stylesheet\" href=\"/?css\">"));
        assert!(!page.contains("/?css2"));
    }

    #[test]
    fn test_css_contains_typography_rules() {
        let css = include_str!("assets/style.css");
        assert!(css.contains("font-family:"));
        assert!(css.contains("max-width:"));
        assert!(css.contains("line-height:"));
    }

    #[test]
    fn test_css_contains_element_rules() {
        let css = include_str!("assets/style.css");
        // Headings
        assert!(css.contains("h1"));
        assert!(css.contains("h6"));
        // Code
        assert!(css.contains("pre {"));
        assert!(css.contains("code {"));
        assert!(css.contains("overflow-x: auto"));
        // Links
        assert!(css.contains("a {"));
        assert!(css.contains("#0366d6"));
        // Blockquotes
        assert!(css.contains("blockquote"));
        assert!(css.contains("#dfe2e5"));
        // HR
        assert!(css.contains("hr {"));
        // Images
        assert!(css.contains("img {"));
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
        assert!(body.contains("id=\"hello\""));
        assert!(body.contains("Hello</h1>"));
        assert!(body.contains("href=\"#hello\""));
    }

    #[test]
    fn test_heading_with_angle_brackets() {
        let body = render_body("# rd::expected<void, E>");
        assert!(body.contains("id=\"rdexpectedvoid-e\""), "got: {}", body);
        // Must NOT double-escape to &amp;lt;
        assert!(!body.contains("&amp;"), "double-escaped: {}", body);
    }

    #[test]
    fn test_render_no_double_escape() {
        let page = render(
            "# rd::expected<void, E>\n\nSome text with <html> & \"quotes\"",
            "test.md",
            false,
        );
        // Title should be escaped once
        assert!(
            page.contains("<title>rd::expected&lt;void, E&gt;</title>"),
            "title: {}",
            page
        );
        // Body heading should be escaped once
        assert!(
            page.contains("id=\"rdexpectedvoid-e\""),
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
        assert!(render_body("## Sub").contains("id=\"sub\">"));
        assert!(render_body("### Sub").contains("id=\"sub\">"));
        assert!(render_body("#### Sub").contains("id=\"sub\">"));
        assert!(render_body("##### Sub").contains("id=\"sub\">"));
        assert!(render_body("###### Sub").contains("id=\"sub\">"));
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

    #[test]
    fn test_double_backtick_inline_code() {
        let body = render_body("`` `code` ``");
        assert!(body.contains("<code>`code`</code>"));
    }

    // === US1: Link rendering ===

    #[test]
    fn test_link() {
        let body = render_body("[click here](https://example.com)");
        assert!(body.contains("<a href=\"https://example.com\">click here</a>"));
    }

    // === Strikethrough ===

    #[test]
    fn test_strikethrough() {
        let body = render_body("~~deleted~~");
        assert!(body.contains("<del>deleted</del>"));
    }

    #[test]
    fn test_strikethrough_with_inline() {
        let body = render_body("~~**bold** deleted~~");
        assert!(body.contains("<del><strong>bold</strong> deleted</del>"));
    }

    // === Auto-linking ===

    #[test]
    fn test_autolink_https() {
        let body = render_body("Visit https://example.com for info");
        assert!(body.contains("<a href=\"https://example.com\">https://example.com</a>"));
    }

    #[test]
    fn test_autolink_strips_trailing_punctuation() {
        let body = render_body("See https://example.com.");
        assert!(body.contains("<a href=\"https://example.com\">https://example.com</a>."));
    }

    #[test]
    fn test_autolink_http() {
        let body = render_body("http://example.com works too");
        assert!(body.contains("<a href=\"http://example.com\">http://example.com</a>"));
    }

    // === Task lists ===

    #[test]
    fn test_task_list_unchecked() {
        let body = render_body("- [ ] todo item");
        assert!(body.contains("<input type=\"checkbox\" disabled> todo item"));
    }

    #[test]
    fn test_task_list_checked() {
        let body = render_body("- [x] done item");
        assert!(body.contains("<input type=\"checkbox\" checked disabled> done item"));
    }

    #[test]
    fn test_task_list_mixed() {
        let body = render_body("- [x] done\n- [ ] pending");
        assert!(body.contains("checked disabled> done"));
        assert!(body.contains("checkbox\" disabled> pending"));
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
        let page = render("# Test\n\nHello world", "test.md", false);
        assert!(page.starts_with("<!DOCTYPE html>"));
        assert!(page.contains("<title>Test</title>"));
        assert!(page.contains("id=\"test\""));
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
        let page = render("", "empty.md", false);
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
        assert!(body.contains("id=\"hello-world--friends\""));
        assert!(body.contains("Hello &lt;world&gt; &amp; &quot;friends&quot;</h1>"));
    }

    #[test]
    fn test_html_escaping_in_list_item() {
        let body = render_body("- item with <html> & stuff");
        assert!(body.contains("<li>item with &lt;html&gt; &amp; stuff</li>"));
    }

    // ===== Table Tests =====

    // Phase 2: Foundational helpers

    #[test]
    fn test_is_table_separator_valid() {
        assert!(is_table_separator("|---|---|"));
        assert!(is_table_separator("| :--- | ---: | :---: |"));
        assert!(is_table_separator("---|---"));
        assert!(is_table_separator("|:---|:---:|---:|"));
    }

    #[test]
    fn test_is_table_separator_invalid() {
        assert!(!is_table_separator("| no dashes |"));
        assert!(!is_table_separator("just some text"));
        assert!(!is_table_separator("| |"));
        assert!(!is_table_separator("||"));
    }

    #[test]
    fn test_split_table_cells_basic() {
        let cells = split_table_cells("| a | b | c |");
        assert_eq!(cells, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_table_cells_no_outer_pipes() {
        let cells = split_table_cells("a | b | c");
        assert_eq!(cells, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_split_table_cells_pipe_in_code() {
        let cells = split_table_cells("| `a|b` | c |");
        assert_eq!(cells, vec!["`a|b`", "c"]);
    }

    #[test]
    fn test_split_table_cells_empty_cell() {
        let cells = split_table_cells("| a || c |");
        assert_eq!(cells, vec!["a", "", "c"]);
    }

    #[test]
    fn test_parse_alignment() {
        let aligns = parse_alignment("| :--- | ---: | :---: | --- |");
        assert_eq!(aligns.len(), 4);
        assert_eq!(aligns[0], Alignment::Left);
        assert_eq!(aligns[1], Alignment::Right);
        assert_eq!(aligns[2], Alignment::Center);
        assert_eq!(aligns[3], Alignment::Left);
    }

    // Phase 3: US1 — Basic Tables

    #[test]
    fn test_table_basic() {
        let body = render_body("| A | B | C |\n|---|---|---|\n| 1 | 2 | 3 |\n| 4 | 5 | 6 |");
        assert!(body.contains("<table>"));
        assert!(body.contains("<thead>"));
        assert!(body.contains("<tbody>"));
        assert!(body.contains("<th>A</th>"));
        assert!(body.contains("<th>B</th>"));
        assert!(body.contains("<th>C</th>"));
        assert!(body.contains("<td>1</td>"));
        assert!(body.contains("<td>6</td>"));
    }

    #[test]
    fn test_table_inline_formatting() {
        let body = render_body("| H |\n|---|\n| **bold** |\n| `code` |\n| [link](url) |");
        assert!(body.contains("<td><strong>bold</strong></td>"));
        assert!(body.contains("<td><code>code</code></td>"));
        assert!(body.contains("<td><a href=\"url\">link</a></td>"));
    }

    #[test]
    fn test_table_header_only() {
        let body = render_body("| A | B |\n|---|---|");
        assert!(body.contains("<table>"));
        assert!(body.contains("<th>A</th>"));
        assert!(body.contains("<th>B</th>"));
        assert!(body.contains("<tbody>"));
    }

    #[test]
    fn test_no_table_without_separator() {
        let body = render_body("| A | B |\n| 1 | 2 |");
        assert!(!body.contains("<table>"));
        assert!(body.contains("<p>"));
    }

    #[test]
    fn test_table_followed_by_paragraph() {
        let body = render_body("| A |\n|---|\n| 1 |\n\nSome text after.");
        assert!(body.contains("<table>"));
        assert!(body.contains("</table>"));
        assert!(body.contains("<p>Some text after.</p>"));
    }

    #[test]
    fn test_table_preceded_by_heading() {
        let body = render_body("# Title\n\n| A |\n|---|\n| 1 |");
        assert!(body.contains("id=\"title\""));
        assert!(body.contains("<table>"));
        assert!(body.contains("<td>1</td>"));
    }

    // Phase 4: US2 — Column Alignment

    #[test]
    fn test_table_alignment() {
        let body = render_body("| L | C | R |\n| :--- | :---: | ---: |\n| a | b | c |");
        assert!(body.contains("<th>L</th>"));
        assert!(body.contains("<th style=\"text-align: center\">C</th>"));
        assert!(body.contains("<th style=\"text-align: right\">R</th>"));
        assert!(body.contains("<td style=\"text-align: center\">b</td>"));
        assert!(body.contains("<td style=\"text-align: right\">c</td>"));
    }

    #[test]
    fn test_table_default_alignment_no_style() {
        let body = render_body("| A |\n|---|\n| 1 |");
        // Default (left) alignment should not have style attribute
        assert!(body.contains("<th>A</th>"));
        assert!(body.contains("<td>1</td>"));
        assert!(!body.contains("text-align: left"));
    }

    // Phase 5: US3 — Styled Tables (CSS)

    #[test]
    fn test_css_contains_table_rules() {
        let css = include_str!("assets/style.css");
        assert!(css.contains("table {"));
        assert!(css.contains("border-collapse: collapse"));
        assert!(css.contains("th, td {"));
        assert!(css.contains("border: 1px solid"));
    }

    // Phase 6: Edge Cases

    #[test]
    fn test_table_fewer_columns_pads() {
        let body = render_body("| A | B | C |\n|---|---|---|\n| 1 |");
        // Row has 1 column, header has 3 — should pad with empty cells
        assert!(body.contains("<td>1</td>"));
        let td_count = body.matches("<td>").count();
        assert_eq!(td_count, 3); // 1 content + 2 empty
    }

    #[test]
    fn test_table_more_columns_truncates() {
        let body = render_body("| A | B |\n|---|---|\n| 1 | 2 | 3 | 4 |");
        // Row has 4 columns, header has 2 — should truncate to 2
        let td_count = body.matches("<td>").count();
        assert_eq!(td_count, 2);
        assert!(!body.contains("<td>3</td>"));
    }

    #[test]
    fn test_table_empty_cells() {
        let body = render_body("| A | B | C |\n|---|---|---|\n| 1 || 3 |");
        assert!(body.contains("<td>1</td>"));
        assert!(body.contains("<td></td>")); // empty cell
        assert!(body.contains("<td>3</td>"));
    }

    #[test]
    fn test_table_pipe_in_code_cell() {
        let body = render_body("| Code |\n|---|\n| `a|b` |");
        assert!(body.contains("<code>a|b</code>"));
        // Should be one cell, not split at the pipe
        let td_count = body.matches("<td>").count();
        assert_eq!(td_count, 1);
    }

    #[test]
    fn test_table_no_outer_pipes() {
        let body = render_body("A | B\n---|---\n1 | 2");
        assert!(body.contains("<table>"));
        assert!(body.contains("<th>A</th>"));
        assert!(body.contains("<td>2</td>"));
    }

    // Spec 6: Asset serving — wrap_html_page with query-string assets

    #[test]
    fn test_wrap_html_page_uses_query_string_assets() {
        let page = wrap_html_page("Title", "<p>hi</p>", false);
        assert!(page.contains("<link rel=\"stylesheet\" href=\"/?css\">"));
        assert!(!page.contains("<style>"));
    }

    // Spec 7: Syntax highlighting

    #[test]
    fn test_wrap_html_page_includes_highlight_js() {
        let page = wrap_html_page("Title", "<p>hi</p>", false);
        assert!(page.contains("<script src=\"/?js\"></script>"));
        assert!(page.contains("hljs.highlightAll()"));
    }

    #[test]
    fn test_asset_consts_not_empty() {
        assert!(!CSS_GZ.is_empty());
        assert!(!HLJS_JS_GZ.is_empty());
        // Gzipped data starts with magic bytes 0x1f 0x8b
        assert_eq!(CSS_GZ[0], 0x1f);
        assert_eq!(CSS_GZ[1], 0x8b);
        assert_eq!(HLJS_JS_GZ[0], 0x1f);
        assert_eq!(HLJS_JS_GZ[1], 0x8b);
    }
}
