use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct AsciiChar {
    pub character: char,
    pub ascii: Vec<String>,
}

fn main() {
    // Specify the output directory for generated files
    let out_dir = "src/compiled_fonts/";

    // List of font files to be processed
    let fonts = vec![
        ("default", "fonts/default.aff"),
        // Add more fonts here
        // ("font_name", "path/to/font_file.aff"),
    ];

    for (name, path) in fonts {
        generate_font_code(name, path, &out_dir);
    }
}

fn generate_font_code(name: &str, path: &str, out_dir: &str) {
    // Read the font file
    let font = fs::read_to_string(path).expect("Failed to read font file");

    // Parse the font file into a vector of AsciiChar
    let mut lines = font.lines();
    let mut ascii_chars = Vec::new();

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

    let generated_code = format!(
        "use crate::cli::AsciiChar;\n\npub static {}_FONT_DATA: &[AsciiChar] = &{:?};",
        name.to_uppercase(),
        ascii_chars
    );

    // Write the generated code to a file
    let dest_path = PathBuf::from(out_dir).join(format!("{}_font.rs", name));
    fs::write(&dest_path, generated_code).expect("Failed to write generated font code");
}
