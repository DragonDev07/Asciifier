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
    let input = input.trim(); // Remove leading and trailing whitespaces
    let mut ascii_art: Vec<String> = Vec::new(); // Declare Vec to store ASCII Art

    // Loop through input string, and find corresponding ASCII Art
    for c in input.chars() {
        let ascii_char = ascii_chars
            .iter()
            .find(|ascii_char| ascii_char.character == c)
            .unwrap();

        // Append ASCII Art to Vector
        for (i, line) in ascii_char.ascii.iter().enumerate() {
            if ascii_art.len() <= i {
                ascii_art.push(String::new());
            }

            ascii_art[i].push_str(line);
        }
    }

    return ascii_art;
}

// ------ Convert & Print ASCII Art ------ //
fn print_ascii(input: &str, ascii_chars: &Vec<AsciiChar>) {
    // Comvert input to ASCII Art
    let ascii_art = convert_to_ascii(input, ascii_chars);

    // Print ASCII Art
    for line in ascii_art {
        println!("{}", line);
    }
}

fn main() {
    let input = "Hello World!";
    let font_path = "fonts/default.txt";
    let font = read_font(font_path);

    print_font(&font);
    print_ascii(&input, &font);
}
