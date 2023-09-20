const KEY_WORDS: [&str; 4] = [";", "{", "}", "="];

fn iter_string(content: &str) -> Vec<String> {
    let mut v = content
        .as_bytes()
        .iter()
        .map(|x| {
            let c = *x as char;
            c.to_string()
        })
        .collect::<Vec<String>>();

    v.retain(|x| x != " " && x != "\t" && x != "\n");

    v
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
