
fn parse_to_double_tags(content: &str, tag: &str) -> String {
    return format!("<{}>{}</{}>", tag, content, tag);
}

pub fn parse_line(line: &str) -> String {
    let trimmed = line.trim();

    // Parse to h1
    if trimmed.starts_with("# ") {
        let content = &trimmed[2..];
        return parse_to_double_tags(content, "h1");
    }

    // Parse to h2
    if trimmed.starts_with("## ") {
        let content = &trimmed[3..];
        return parse_to_double_tags(content, "h2");
    }

    // Parse to h3
    if trimmed.starts_with("### ") {
        let content = &trimmed[4..];
        return parse_to_double_tags(content, "h3");
    }

    // Parse to h4
    if trimmed.starts_with("#### ") {
        let content = &trimmed[5..];
        return parse_to_double_tags(content, "h4");
    }

    // Parse to h5
    if trimmed.starts_with("##### ") {
        let content = &trimmed[6..];
        return parse_to_double_tags(content, "h5");
    }

    // Parse to h6
    if trimmed.starts_with("###### ") {
        let content = &trimmed[7..];
        return parse_to_double_tags(content, "h6");
    }

    if trimmed.is_empty() {
        return String::new();
    }

    return parse_to_double_tags(trimmed, "p");
}

pub fn create_html_document(html_lines: Vec<String>) -> String {
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"UTF-8\">\n
            <title>Converted Document</title>\n</head>\n<body>\n{}\n</body>\n</html>",
        html_lines.join("\n")
    )
}
