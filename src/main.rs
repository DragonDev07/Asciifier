use clap::Parser;

const DEFAULT_FONT: &str = include_str!("fonts/default.txt");

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Teo Welton <teowelton@gmail.com>")]
struct Args {
    #[arg(short = 'i', long = "input", help = "String to convert to ASCII Art")]
    input: Option<String>,

    #[arg(
        short = 'f',
        long = "font",
        help = "name of a built-in font or path to a font file. IF provided without -i, it will print the font"
    )]
    font: Option<String>,
}

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
fn read_font(font: &str) -> Vec<AsciiChar> {
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

// ------ Read font from file ------ //
fn read_font_from_file(path: &str) -> Vec<AsciiChar> {
    println!("Reading font from file: {}", &path);
    let font = std::fs::read_to_string(path).expect("Failed to read font file");
    return read_font(&font);
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
    // Parse Command Line Arguments using clap
    let args = Args::parse();

    // If neither font or input is provided, print usage
    if args.font.is_none() && args.input.is_none() {
        println!("USAGE GOES HERE");
        return;
    }

    // Function to get the font
    fn get_font(font_option: &Option<String>) -> Vec<AsciiChar> {
        match font_option {
            Some(font) if font.to_ascii_lowercase() == "default" => read_font(DEFAULT_FONT),
            Some(font) => read_font_from_file(&font),
            None => read_font(DEFAULT_FONT),
        }
    }

    // Get the font
    let ascii_chars: Vec<AsciiChar> = get_font(&args.font);

    // If input is provided, print it with the chosen font
    if let Some(input) = args.input {
        print_ascii(&input, &ascii_chars);
    } else {
        // If input is not provided, but font is, print font
        print_font(&ascii_chars);
    }
}