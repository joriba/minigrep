use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = 
        if config.ignore_case { 
            search_case_insensitive(&config.query, &contents)
        } else {
            search_case_sensitive(&config.query, &contents)
        };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line)
        }
    }

    results
}

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub ignore_case: bool
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, String> {
        // want total of 3 args: binary location (always included), search string, input file
        if args.len() != 3 {
            return Err(format!(
                "Wrong number of arguments! 2 required, but {} {} found",
                args.len() - 1,
                if args.len() == 2 { "was" } else { "were" }
            ));
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
