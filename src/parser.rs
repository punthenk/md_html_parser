
fn parse_to_double_tags(content: &str, tag: &str) -> String {
    return format!("<{}>{}</{}>", tag, content, tag);
}

fn check_header(line: &str) -> String {
    let count = line.chars().filter(|c| *c == '#').count();
    let content = &line[count+1..];
    let tag = "h".to_owned() + &count.to_string();

    return parse_to_double_tags(&content, &tag);
}

fn check_italic_or_bold(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0; 
    let mut start_index = 0;
    let mut stop_index = 0;

    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            start_index = i+2;
            println!("Start BOLD at {i}");
            i += 2;

            while i + 1 < chars.len() {
                if chars[i] == '*' && chars[i + 1] == '*' {
                    stop_index = i;
                    println!("End BOLD at {i}");

                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        i += 1;
    }

    let content = &line[start_index..stop_index];
    return parse_to_double_tags(content, "b");
}

pub fn parse_line(line: &str) -> String {
    let trimmed = line.trim();

    if trimmed.starts_with('#') {
        return check_header(trimmed);
    }

    if trimmed.contains('*') {
        return check_italic_or_bold(trimmed);
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
