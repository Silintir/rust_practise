use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {} ...", _filename);

    let input_file = Path::new(_filename);

    let file = File::open(&input_file).expect("[ ERROR ] Failed to open a file");

    let mut _ptag = false;
    let mut _htag = false;

    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }

                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }

                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }

        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output = String::from(&_filename[.._filename.len() - 2]);
    output.push_str("html");

    let mut outfile = File::create(output).expect("[ ERROR ] Could not create output file!");

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("usage: {} <somefile>.md", env!("CARGO_PKG_NAME"));
}

fn print_short_banner() {
    println!("{}", get_title());
}

fn get_title() -> String {
    let mut thr_title = String::from(env!("CARGO_PKG_NAME"));
    thr_title.push_str(" (v");
    thr_title.push_str(env!("CARGO_PKG_VERSION"));
    thr_title.push_str("), ");
    thr_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    thr_title.push_str("A tiny markdown compiler.");
    thr_title
}

fn usage() {
    print_long_banner()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation");
            usage();
        }
    }
}
