use std::fs::File;
use std::io::{BufRead, BufReader};

struct AsciiChar {
    character: char,
    ascii: Vec<String>,
}

// ------ Print Font Function ------ //
fn print_font(ascii_chars: &Vec<AsciiChar>) {
    for ascii_char in ascii_chars {
        println!("Character: {}", ascii_char.character);
        println!("ASCII Art:");
        for line in &ascii_char.ascii {
            println!("{}", line);
        }
        println!("------------------");
    }
}

// ------ Read Font Function, given a path to font file, return a Vector of Type AsciiChar ------ //
fn read_font(font_path: &str) -> Vec<AsciiChar> {
    let file = File::open(font_path).expect("Unable to open file"); // Open given file
    let reader = BufReader::new(file); // Declare BufReader to read file
    let mut lines = reader.lines(); // Declare lines iterator

    let mut ascii_chars: Vec<AsciiChar> = Vec::new(); // Declare Vec to store AsciiChars

    // Loop through lines, and create AsciiChar structs
    while let Some(Ok(character_line)) = lines.next() {
        let character = character_line.chars().next().unwrap();
        let mut ascii = Vec::new();

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }
            ascii.push(line);
        }

        ascii_chars.push(AsciiChar { character, ascii });
    }

    return ascii_chars;
}

// ------ Convert Input String to ASCII Art ------ //
fn convert_to_ascii(input: &str, ascii_chars: &[AsciiChar]) -> Vec<String> {
    let mut ascii_art: Vec<Vec<String>> = Vec::new();

    for line in input.lines() {
        ascii_art.push(Vec::new());
        for ch in line.chars() {
            if ch.is_whitespace() {
                for ascii_line in ascii_art.last_mut().unwrap() {
                    ascii_line.push_str(" ");
                }
            } else {
                let ascii_char = ascii_chars.iter().find(|&c| c.character == ch);
                if let Some(ascii_char) = ascii_char {
                    while ascii_art.last().unwrap().len() < ascii_char.ascii.len() {
                        ascii_art.last_mut().unwrap().push(String::new());
                    }
                    for (i, ascii_line) in ascii_char.ascii.iter().enumerate() {
                        ascii_art.last_mut().unwrap()[i].push_str(ascii_line);
                    }
                }
            }
        }
    }

    ascii_art.into_iter().map(|v| v.join("\n")).collect()
}

// ------ Print Converted ASCII Art ------ //
fn print_ascii(ascii_art: &Vec<String>) {
    for line in ascii_art {
        println!("{}", line);
    }
}

fn main() {    
    let input = "abc";
    let font_path = "fonts/default.txt";
    let ascii_chars = read_font(font_path);
    let ascii_art = convert_to_ascii(input, &ascii_chars);

    print_font(&ascii_chars);
    print_ascii(&ascii_art);
}
