use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ? is to handle Result, return OK() if no error, else return Err from the whole function
    let content = fs::read_to_string(config.file_path)?;
    for found_line in search(&config.query, &content) {
        println!("{found_line}");
    }
    Ok(())
}

//  indicate that the returned vector should contain string slices that
// reference slices of the argument contents (rather than the argument query).
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    // Returns an iterator over the lines of a string, as string slices.
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
