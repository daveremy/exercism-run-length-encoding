use itertools::Itertools;

pub fn encode(source: &str) -> String {
    source
        .chars()
        .group_by(|c| *c)
        .into_iter()
        .map(|(c, group)| encode_char(c, group.count()))
        .join("")
}

pub fn encode_char(c: char, count: usize) -> String {
    match count {
        1 => c.to_string(),
        _ => format!("{}{}", count, c),
    }
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut num_accum = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            num_accum.push(c);
        } else {
            let count = num_accum.parse().unwrap_or(1);
            decoded += &c.to_string().repeat(count);
            num_accum.clear();
        }
    }
    decoded
}
