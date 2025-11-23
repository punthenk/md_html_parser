use clap::{Parser};
use std::{fs, path::Path};
use std::ffi::OsStr;

mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(short, long)]
    output_file: Option<String>,
}

fn get_extention_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}

fn main() {
    let args = Args::parse();
    let filename = &args.file;

    if get_extention_from_filename(filename) != Some("md") {
        println!("This is not the right file type");
        println!("Make sure you use a .md file");
        return;
    }

    println!("Reading {}...\n", filename);


    let contents: String = fs::read_to_string(filename)
        .expect("Can't read the file");

    println!("File loaded successfully!\n");

    let html_lines: Vec<String> = contents
        .lines() // Split the string into lines
        .map(|line| parser::parse_line(line)) // calls parse_line on each line
        .collect(); // Collect the lines into a Vec
    
    let html_output = parser::create_html_document(html_lines);

    if let Some(filename) = args.output_file {
        if get_extention_from_filename(&filename) != Some("html") {
            println!("We can only write in html files");
            return;
        }
        
        fs::write(&filename, &html_output)
            .expect("Can't write to output file");

        println!("Successfully written to {}", filename);
    }
}
