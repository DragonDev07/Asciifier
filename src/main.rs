use std::fs::File;
use std::io::{self, BufRead};

struct AsciiChar {
    character: char,
    ascii_art: Vec<String>,
}

fn read_ascii_font(filename: &str) -> Vec<AsciiChar> {
    let file = File::open(filename).unwrap();
    let reader = io::BufReader::new(file);
    let mut ascii_chars: Vec<AsciiChar> = Vec::new();
    let mut lines = reader.lines();
    let mut character = ' ';
    let mut ascii_art: Vec<String> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        if line.len() == 1 {
            if character != ' ' {
                ascii_chars.push(AsciiChar {
                    character: character,
                    ascii_art: ascii_art,
                });
            }
            character = line.chars().next().unwrap();
            ascii_art = Vec::new();
        } else {
            ascii_art.push(line);
        }
    }
    return ascii_chars;
}

fn main() {
    // String to Convert (Temporary)
    // TODO: Read as an arg
    let input = "ABC";

    // Read the ascii font to a vector named "font"
    let font = read_ascii_font("fonts/default.txt");

    // Convert the input string to ascii art
    let mut ascii_art: Vec<String> = Vec::new();
    for line in 0..font[0].ascii_art.len() {
        let mut ascii_line = String::new();
        for character in input.chars() {
            let ascii_char = font.iter().find(|&c| c.character == character).unwrap();
            ascii_line.push_str(&ascii_char.ascii_art[line]);
        }
        ascii_art.push(ascii_line);
    }

    // Print the ascii art
    for line in ascii_art {
        println!("{}", line);
    }
}
