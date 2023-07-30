use clap::{Parser};
use figlet_rs::FIGfont;

#[derive(Parser, Debug)]
struct FigletCtl {
    message: String,
}

fn main() {
    let args = FigletCtl::parse();
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert(args.message.as_str());
    println!("{}", figure.unwrap());
}
