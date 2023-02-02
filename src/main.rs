use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
        
    println!("With text:\n{}", contents);
}

struct Config <'a>{
    query: &'a str,
    file_path: &'a str,
}

impl <'a> Config<'a> {
    fn build(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(format!("Wrong number of arguments! 2 required, but {} {} found", args.len() - 1,
                if args.len() == 2 {"was"} else {"were"}));
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Config { query, file_path })
    }
}