fn parse_markdown_file( _filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {} ...", _filename);

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
    thr_title.push_str("A tiny markdown compiler based on Jesse's tutorials.");
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