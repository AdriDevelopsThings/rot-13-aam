const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const CHANGE_VALUE: i8 = 13;

enum Method {
    Encrypt,
    Decrypt,
}

fn rot13(input: &str, method: Method) -> String {
    let chars = ALPHABET.chars().collect::<Vec<char>>();
    let mut output = String::new();

    for char in input.chars() {
        let char_index = char.to_lowercase().next().unwrap();
        output += &match chars.iter().position(|a: &char| *a == char_index) {
            Some(position) => {
                let mut position = match method {
                    Method::Encrypt => (position as i8) + CHANGE_VALUE,
                    Method::Decrypt => (position as i8) - CHANGE_VALUE,
                };

                if position >= ALPHABET.len() as i8 {
                    position -= ALPHABET.len() as i8;
                } else if position < 0 {
                    position += ALPHABET.len() as i8;
                }

                if char.is_uppercase() {
                    chars[position as usize].to_uppercase().to_string()
                } else {
                    chars[position as usize].to_string()
                }
            }
            None => char.to_string(),
        };
    }

    output
}

pub fn encrypt(input: &str) -> String {
    rot13(input, Method::Encrypt)
}

pub fn decrypt(input: &str) -> String {
    rot13(input, Method::Decrypt)
}
