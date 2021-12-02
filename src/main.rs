mod solutions;
mod error;
pub use error::Error;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Unexpected number of args received. Usage: \n ./knowit <1-24>");
        process::exit(-1);
    }
    let luke = &args[1];
    if let Ok(i) = luke.parse::<u8>() {
        if i < 1 || i > 1 {
            eprint!("The number you wanted to solve is currently not implemented. ");
            process::exit(-1)
        }
        if let Ok(result) = solutions::solver(1) {
            println!("Solution to luke {}: {}",i ,result);
        }
    }
}
