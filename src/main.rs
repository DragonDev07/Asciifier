use clap::Parser;

// ------ Import Other Modules ------ //
mod args;
mod font;

fn main() {
    // Parse Command Line Arguments using clap
    let args = args::Args::parse();

    // If neither font or input is provided, print usage
    if args.font.is_none() && args.input.is_none() {
        println!("USAGE GOES HERE");
        return;
    }

    // Get the font
    let ascii_chars: Vec<font::AsciiChar> = font::get_font(&args.font);

    if let Some(input) = args.input {
        // If input is provided, print it with the chosen font
        font::print_ascii(&input, &ascii_chars);
    } else {
        // If no input is provided, print what is read from the font
        font::print_font(&ascii_chars);
    }
}
