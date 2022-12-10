use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 | 2 => panic!("Please add an argument"),
        3 => {
            println!("Transforming argument");
            let shift: u8 = args[2].parse().unwrap();
            println!(
                "{} --> {}",
                args[1],
                caesar_decrypt(args[1].as_str(), shift)
            );
        }
        _ => panic!("Too many arguments"),
    }
}

pub fn caesar_encrypt(word: &str, shift: u8) -> String {
    word.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let start = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (start + (c as u8 + shift - start) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_decrypt(word: &str, shift: u8) -> String {
    word.chars()
        .map(|c| {
            if c.is_alphabetic() {
                let start = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let mut shifted_c = c as u8 - shift;
                if shifted_c < start {
                    shifted_c += 26;
                }
                shifted_c as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_encrypt() {
        assert_eq!(caesar_encrypt("", 13), "");
    }

    #[test]
    fn caesar_rot_13_encrypt() {
        assert_eq!(caesar_encrypt("rust", 13), "ehfg");
    }

    #[test]
    fn empty_decrypt() {
        assert_eq!(caesar_decrypt("", 13), "");
    }

    #[test]
    fn caesar_rot_13_decrypt() {
        assert_eq!(caesar_decrypt("ehfg", 13), "rust");
    }
}
