use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();

    println!("FILE is = {}", args.file);

    let contents = fs::read_to_string(args.file)
        .expect("Can't read the file");

    println!("contents:\n{contents}");
    
}
