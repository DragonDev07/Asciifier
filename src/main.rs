mod args;
mod cli;

fn main() {
    let matches = args::args(); // Get Command Arguments
    let ascii_chars: Vec<cli::AsciiChar> = args::handle_font(&matches); // Get Font
    args::handle_debug(&matches, &ascii_chars); // Handle Debug Option

    if let Some(text) = matches.get_one::<String>("text") {
        cli::print_ascii(&text, &ascii_chars); // Print Converted Ascii
    }
}
