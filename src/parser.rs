
fn parse_to_double_tags(content: &str, tag: &str) -> String {
    return format!("<{}>{}</{}>", tag, content, tag);
}

fn check_header(line: &str) -> String {
    let count = line.chars().filter(|c| *c == '#').count();
    let content = &line[count+1..];
    let tag = "h".to_owned() + &count.to_string();

    return parse_to_double_tags(&content, &tag);
}

pub fn parse_line(line: &str) -> String {
    let trimmed = line.trim();

    if trimmed.starts_with("#") {
        return check_header(trimmed);
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
