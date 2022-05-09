use std::env;
use std::error::Error;
use std::fs;
use ansi_term::Colour::Green;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents) 
    }
    else {
         search_case_insensitive(&config.query, &contents)  
    };
    
    for line in results {

        if config.case_sensitive == true {
            
            let colorized_start = line.find(&config.query).unwrap();
        let colorized_stop = colorized_start + config.query.len();

        println!("{}{}{}", &line[..colorized_start],
        Green.bold().paint(&config.query),
        &line[colorized_stop..]);} 
        

        else {
            let colorized_start = line.to_lowercase().find(&config.query.to_lowercase()).unwrap();
        let colorized_stop = colorized_start + config.query.len();

        println!("{}{}{}", &line[..colorized_start],
        Green.bold().paint(&config.query.to_lowercase()),
        &line[colorized_stop..]);}

        }
        
        

    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        if args.len() == 4 && args[3] == "-h" {
            println!("
    This app searches outputs lines of text file,
    containing the first match to a query.
    Pass args as follows: minigrep.exe query path-to-file [i]
    Where [i] is a flag for case-insensitive search.
    You can also use ENV variable CASE_INSENSITIVE
    To allow case-insensitive search by default.\n");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        if !env::var("CASE_INSENSITIVE").is_err() || (args.len() == 4 && args[3] == "i") {

            let case_sensitive = false;
            Ok(Config {
                query, 
                filename, 
                case_sensitive,
            })
        }
        
        else {
            let case_sensitive = true;
            Ok(Config {
                query, 
                filename, 
                case_sensitive,
            })
        }

    }
}

fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result: Vec<&'a str> = Vec::new();

    for line in contents.lines() {

        if line.contains(query) {
            result.push(line)
        } 

    }

    result
}

fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> { 
    
    let mut result: Vec<&'a str> = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {

        if line.to_lowercase().contains(&query) {
            result.push(line)
        } 
                
    }

    result
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search_case_insensitive(query, contents));
    }

}