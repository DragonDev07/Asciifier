use std::fs::File;
use std::io::{BufRead, BufReader};

// - [ ] Read Font to Vector with characters + ascii art as a struct or touple
// - [ ] Loop through input string, and convert to ascii
//   - INCLUDE INTERPRETATION OF SPACES, NEW LINES, TABS, ETC.
// - [ ] Loop through and print out the ascii representation

struct AsciiChar {
    character: char,
    ascii: Vec<String>,
}

// ------ Print Font Function ------ //
fn print_font(ascii_chars: Vec<AsciiChar>) {
    for ascii_char in ascii_chars {
        println!("Character: {}", ascii_char.character);
        println!("ASCII Art:");
        for line in ascii_char.ascii {
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

fn main() {
    let font_path = "fonts/default.txt";
    let ascii_chars = read_font(font_path);
    print_font(ascii_chars);
}
