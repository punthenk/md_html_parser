pub fn parse_to_double_tags(content: &str, tag: &str) -> String {
    return format!("<{}>{}</{}>", tag, content, tag);
}

pub fn check_header(line: &str) -> String {
    let count = line.chars().filter(|c| *c == '#').count();
    let content = &line[count+1..];
    let tag = "h".to_owned() + &count.to_string();

    return parse_to_double_tags(&content, &tag);
}

pub fn check_italic_or_bold(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut i = 0; 
    let mut out = String::new();

    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == '*' && chars[i + 1] == '*' {
            i += 2;

            let start = i;

            while i + 1 < chars.len() {
                if chars[i] == '*' && chars[i + 1] == '*' {
                    let end = i;
                    let content: String = chars[start..end].iter().collect();

                    out.push_str("<b>");
                    out.push_str(&content);
                    out.push_str("</b>");

                    i += 2;
                    break;
                }
                i += 1;
            }
            continue;
        }

        if i + 1 < chars.len() && chars[i] == '*' {
            i += 1;
            let start = i;

            while i < chars.len() {
                if chars[i] == '*' {
                    let end = i;
                    let content: String = chars[start..end].iter().collect();

                    out.push_str("<i>");
                    out.push_str(&content);
                    out.push_str("</i>");

                    i += 1;
                    break;
                }
                i += 1;
            }
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    return parse_to_double_tags(&out, "p");
}

pub fn check_unordered_list(line: &str) -> String {
    let mut out: String = String::new();
    out.push_str("<ul>\n");

    let content = &line[2..];
    let new_content = parse_to_double_tags(content, "li");

    out.push_str(&new_content);
    out.push_str("\n</ul>");

    return out;
}
