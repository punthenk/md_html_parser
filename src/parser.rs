use crate::parse_line::{ check_header, check_italic_or_bold, check_unordered_list, parse_to_double_tags };

pub fn parse_line(line: &str) -> String {
    let trimmed = line.trim();

    if trimmed.starts_with('#') {
        return check_header(trimmed);
    }

    if trimmed.contains('*') {
        return check_italic_or_bold(trimmed);
    }

    if trimmed.starts_with('-') {
        return check_unordered_list(trimmed);
    }

    if trimmed.is_empty() {
        return String::new();
    }

    return parse_to_double_tags(trimmed, "p");
}

pub fn create_html_document(html_lines: Vec<String>) -> String {
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n  <meta charset=\"UTF-8\">\n  <title>Converted Document</title>\n</head>\n<body>\n{}\n</body>\n</html>",
        html_lines.join("\n")
    )
}
