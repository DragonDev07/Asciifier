use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

struct AsciiChar {
    character: char,
    ascii_art: Vec<String>,
}

fn read_ascii_font(filename: &str) -> HashMap<char, AsciiChar> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut ascii_chars: HashMap<char, AsciiChar> = HashMap::new();
    let mut lines = reader.lines();
    let mut character = ' ';
    let mut ascii_art: Vec<String> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.len() == 1 {
            if character != ' ' {
                ascii_chars.insert(
                    character,
                    AsciiChar {
                        character: character,
                        ascii_art: ascii_art.clone(),
                    },
                );
            }
            character = line.chars().next().unwrap();
            ascii_art.clear();
        } else {
            ascii_art.push(line);
        }
    }
    ascii_chars
}

fn string_to_ascii(input: &str, ascii_chars: &HashMap<char, AsciiChar>) {
    let mut ascii_string: Vec<&AsciiChar> = Vec::new();
    let mut max_height = 0;
    for ch in input.chars() {
        if let Some(ascii_char) = ascii_chars.get(&ch) {
            max_height = max_height.max(ascii_char.ascii_art.len());
            ascii_string.push(ascii_char);
        }
    }
    for i in 0..max_height {
        for ascii_char in &ascii_string {
            let height_diff = max_height - ascii_char.ascii_art.len();
            if i < height_diff {
                print!("{}", " ".repeat(ascii_char.ascii_art[0].len()));
            } else {
                print!("{}", ascii_char.ascii_art[i - height_diff]);
            }
        }
        println!();
    }
}

fn main() {
    let ascii_chars = read_ascii_font("fonts/default.txt");
    let input = "ABC";
    string_to_ascii(input, &ascii_chars);
}
