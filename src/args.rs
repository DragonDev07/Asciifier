use crate::cli;
use clap::{arg, command, value_parser, ArgAction};

/// `args()` - Define the command arguments
pub fn args() -> clap::ArgMatches {
    let matches = command!()
        .arg_required_else_help(true)
        .arg(arg!([text] "The text to convert to ASCII").required(true))
        .arg(
            arg!(-f --font <FONT> "ASCII Font to use - OPTIONS: 'default', 'bulbhead', 'epic', 'arrows', 'banner', 'diet-cola', <path to .aff>'")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!(-d --debug "Enables debug mode")
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .get_matches();

    return matches;
}

/// `handle_debug()` - Handle if the debug option is provided
pub fn handle_debug(matches: &clap::ArgMatches, ascii_chars: &Vec<cli::AsciiChar>) {
    if matches.get_flag("debug") {
        cli::print_font(&ascii_chars);
    }
}

/// `handle_font() - Get font based on command arguments`
/// - If no font is provided, will return default font
/// - If the name of a built-in font is provided, will read and return the associated one
/// - Otherwise, it will assume a path to a font is provided, and attempt to read it
pub fn handle_font(matches: &clap::ArgMatches) -> Vec<cli::AsciiChar> {
    if let Some(font) = matches.get_one::<String>("font") {
        match font.to_lowercase().as_str() {
            "default" => cli::read_font(cli::DEFAULT_FONT),
            "bulbhead" => cli::read_font(cli::BULBHEAD_FONT),
            "epic" => cli::read_font(cli::EPIC_FONT),
            "arrows" => cli::read_font(cli::ARROW_FONT),
            "banner" => cli::read_font(cli::BANNER_FONT),
            "diet-cola" => cli::read_font(cli::DIET_COLA_FONT),
            _ => cli::read_font_from_file(font),
        }
    } else {
        return cli::read_font(cli::DEFAULT_FONT);
    }
}
