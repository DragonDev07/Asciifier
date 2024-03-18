use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

// Define a struct to hold an ASCII art character
struct AsciiChar {
    ascii_art: Vec<String>,
}

// ------ Function to Read ASCII font from file ------ //
fn read_ascii_font(filename: &str) -> HashMap<char, AsciiChar> {
    let file = File::open(filename).unwrap(); // Open File
    let reader = io::BufReader::new(file); // Declare BufReader
    let mut lines = reader.lines(); // Lines iterator

    // Create a HashMap to hold the ASCII characters
    let mut ascii_chars: HashMap<char, AsciiChar> = HashMap::new();

    // Initialize the current character and its ASCII art
    let mut character = ' ';
    let mut ascii_art: Vec<String> = Vec::new();

    // Loop over the lines in the file
    while let Some(Ok(line)) = lines.next() {
        // If the line contains a single character, it's the start of a new ASCII character
        if line.len() == 1 {
            // If we have an ASCII character, add it to the HashMap
            if character != ' ' {
                ascii_chars.insert(
                    character,
                    AsciiChar {
                        ascii_art: ascii_art.clone(),
                    },
                );
            }

            // Update the current character and clear the ASCII art
            character = line.chars().next().unwrap();
            ascii_art.clear();
        } else {
            // Otherwise, the line is part of the ASCII art for the current character
            ascii_art.push(line);
        }
    }
    // Return the HashMap of ASCII characters
    return ascii_chars;
}

// ------ Function to Convert Given String to ASCII Art ------ //
fn string_to_ascii(input: &str, ascii_chars: &HashMap<char, AsciiChar>) {
    let mut ascii_string: Vec<&AsciiChar> = Vec::new(); // Vector to hold ASCII Chars
    let mut max_height = 0; // Max Height of ASCII art value

    // Loop over the characters in the input string
    for ch in input.chars() {
        // If the character is in the HashMap, add it to the vector and update the maximum height
        if let Some(ascii_char) = ascii_chars.get(&ch) {
            max_height = max_height.max(ascii_char.ascii_art.len());
            ascii_string.push(ascii_char);
        }
    }

    // Loop over the lines in the ASCII art
    for i in 0..max_height {
        // Loop over the ASCII characters in the vector
        for ascii_char in &ascii_string {
            // Calculate the difference in height between the maximum height and the height of the ASCII art
            let height_diff = max_height - ascii_char.ascii_art.len();

            // If the ASCII art doesn't have a line at the current index, print spaces
            if i < height_diff {
                print!("{}", " ".repeat(ascii_char.ascii_art[0].len()));
            } else {
                // Otherwise, print the line of the ASCII art
                print!("{}", ascii_char.ascii_art[i - height_diff]);
            }
        }
        // Print a newline character to move to the next line
        println!();
    }
}

fn main() {
    let ascii_chars = read_ascii_font("fonts/default.txt");
    let input = "A\nB";
    string_to_ascii(input, &ascii_chars);
}