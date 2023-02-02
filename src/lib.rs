use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    println!("Searching for \"{}\"", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;
        
    println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config <'a>{
    pub query: &'a str,
    pub file_path: &'a str,
}

impl <'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(format!("Wrong number of arguments! 2 required, but {} {} found", args.len() - 1,
                if args.len() == 2 {"was"} else {"were"}));
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Config { query, file_path })
    }
}