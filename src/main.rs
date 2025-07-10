use minigrep::Config;
use std::env;
use std::process;

/*
Calling the command line parsing logic with the argument values
Setting up any other configuration
Calling a run function in lib.rs
Handling the error if run returns an error
 */
fn main() {
    // args() returns an iterator
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    match minigrep::run(config) {
        Err(e) => {
            println!("Application error: {e}");
            process::exit(1);
        }
        _ => (),
    }
    // // with shorthanded syntax if-let
    // if let Err(e) = run(config) {
    //     println!("Application error: {e}");
    //     process::exit(1);
    // }
}
