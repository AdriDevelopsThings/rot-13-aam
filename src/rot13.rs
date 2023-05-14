fn transform_char(c: u8) -> u8 {
    (c + 13) % 26
}

pub fn rot13(input: &str) -> String {
    let vec: Vec<u8> = input
        .as_bytes()
        .iter()
        .map(|&c| match c {
            b'a'..=b'z' => transform_char(c - b'a') + b'a',
            b'A'..=b'Z' => transform_char(c - b'A') + b'A',
            _ => c,
        })
        .collect();

    String::from_utf8(vec).expect("invalid input string")
}
