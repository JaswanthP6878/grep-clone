use std::{env, process};
use minigrep::Config;
fn main() {
    // let args: Vec<String> = env::args().collect();
    let config  =Config::build(env::args()).unwrap_or_else(|err| {
        println!("Error occured during runtime: {}",err);
        process::exit(2)
    });
    if let Err(e)  = minigrep::run(config){
        println!("Application Error {e}");
        process::exit(1)
    }
}




