use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments! 2 required, but {} {} given", args.len() - 1, 
                if args.len() == 2 {"was"} else {"were"});
        }
        let query = &args[1];
        let file_path = &args[2];
        Config { query, file_path }
    }
}