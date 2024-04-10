use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", author = "Teo Welton <teowelton@gmail.com>")]
pub struct Args {
    #[arg(short = 'i', long = "input", help = "String to convert to ASCII Art")]
    pub input: Option<String>,

    #[arg(
        short = 'f',
        long = "font",
        help = "name of a built-in font or path to a font file. IF provided without -i, it will print the font"
    )]
    pub font: Option<String>,
}