use clap::Parser;
use figlet_rs::Toilet;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 42)]
    tick_rate: u16,
}

fn main() {
    let mono_12_font = Toilet::mono9().unwrap();
    println!("{}", mono_12_font.convert("Langton's Ants").unwrap());
}
