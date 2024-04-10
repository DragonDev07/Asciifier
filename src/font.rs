const DEFAULT_FONT: &str = include_str!("fonts/default.txt");

pub struct AsciiChar {
    pub character: char,
    pub ascii: Vec<String>,
}

// ------ Function to print everything that is read from a font ------ //
pub fn print_font(ascii_chars: &Vec<AsciiChar>) {
    for ascii_char in ascii_chars {
        println!("Character: {}", ascii_char.character);
        println!("ASCII Art:");
        for line in &ascii_char.ascii {
            println!("{}", line);
        }
        println!("------------------");
    }
}

// ------ Function to read font to a Vector of AsciiChars ------ //
pub fn read_font(font: &str) -> Vec<AsciiChar> {
    let mut lines = font.lines(); // Declare lines iterator

    let mut ascii_chars: Vec<AsciiChar> = Vec::new(); // Declare Vec to store AsciiChars

    // Loop through lines, and create AsciiChar structs
    while let Some(character_line) = lines.next() {
        let character = character_line.chars().next().unwrap();
        let mut ascii = Vec::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            ascii.push(line.to_string());
        }

        ascii_chars.push(AsciiChar { character, ascii });
    }

    return ascii_chars;
}

// ------ Function to convert input to AsciiChars ------ //
pub fn convert_to_ascii(input: &str, ascii_chars: &[AsciiChar]) -> Vec<String> {
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

// ------ Function to read font from file ------ //
pub fn read_font_from_file(font_path: &str) -> Vec<AsciiChar> {
    let font = std::fs::read_to_string(font_path).expect("Could not read font file");
    return read_font(&font);
}

// ------ Function to get font based on font_option arg ------ //
pub fn get_font(font_option: &Option<String>) -> Vec<AsciiChar> {
    match font_option {
        Some(font) if font.to_ascii_lowercase() == "default" => read_font(DEFAULT_FONT),
        Some(font) => read_font_from_file(&font),
        None => read_font(DEFAULT_FONT),
    }
}

// ------ Function to convert & print ASCII Art ------ //
pub fn print_ascii(input: &str, ascii_chars: &Vec<AsciiChar>) {
    // Comvert input to ASCII Art
    let ascii_art = convert_to_ascii(input, ascii_chars);

    // Print ASCII Art
    for line in ascii_art {
        println!("{}", line);
    }
}