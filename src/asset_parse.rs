const KEY_WORDS: [&str; 4] = [";", "{", "}", "="];
const WHITE_WORDS: [&str; 3] = [" ", "\t", "\n"];

fn iter_string(content: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    let mut skip = false;
    for v in content.as_bytes() {
        let c = *v as char;
        let s = c.to_string();

        if &s == "#" {
            skip = true;
        }
        if &s == "\n" && skip {
            skip = false;
        }

        if skip {
            continue;
        }

        if WHITE_WORDS.contains(&s.as_str()) {
            continue;
        }

        result.push(s);
    }

    result
}

pub fn to_tokens(content: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut cursor = String::new();

    for i in iter_string(content) {
        let c = i.as_str();

        if KEY_WORDS.contains(&c) {
            if !cursor.is_empty() {
                v.push(cursor.clone());
            }

            v.push(i);
            cursor.clear();
        } else {
            cursor.push_str(&i);
        }
    }

    v
}
