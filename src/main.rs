use clap::{parser, Parser};

fn main() {
    let pattern = std::env::args().nth(1).expect("No pattern given");
    let path = std::env::args().nth(2).expect("No path given");

    let args = Cli::parse();
    println!("pattern : {:?} , path {:?}",args.pattern,args.path);
}

#[derive(Parser)]
struct Cli {
    pattern : String,
    path : std::path::PathBuf,
}

