use std::{io::{self, Read},env, fs, process::ChildStdout};
use std::error::Error;
use ansi_term::Colour::Green;

/*
* The use of this console program is as follows
* 
* minigrep QUERY FILENAME MODIFIER
* 
* OR
*
* A | minigrep QUERY MODIFIER
*
*
*/


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.source).unwrap_or(get_pipe_output());

    let results = if config.case_sensitive {
        search(&config.query, &contents) 
    }
    else {
         search_case_insensitive(&config.query, &contents)  
    };
    
    for line in results {
        
        let query_len = config.query.len();

        if config.case_sensitive == true {
            
            let (all_matches, _):(Vec<_>,Vec<_>) = line.match_indices(&config.query).unzip();

            let mut i = 0;

            while i < line.len() {
                if all_matches.contains(&i) {

                    let end = i+query_len;
                    print!("{}", Green.bold().paint(&line[i..end]));
                    i = end;
                    }
                else { 
                    print!("{}",&line[i..i+1]); 
                    i += 1; 
                }
                    
            }
            println!("");
        }   
        
        else {
            let (all_matches, _):(Vec<_>,Vec<_>) = line.to_lowercase().match_indices(
                &config.query.to_lowercase()).unzip();

            let mut i = 0;

            while i < line.len() {
                if all_matches.contains(&i) {

                    let end = i+query_len;
                    print!("{}", Green.bold().paint(&line[i..end]));
                    i = end;
                    }
                else { 
                    print!("{}",&line[i..i+1]); 
                    i += 1; 
                }
                    
            }
            println!("");


        }

    }           
         
    Ok(())
}

pub struct Config{
    pub query: String,
    pub source: String,
    pub case_sensitive: bool,
    pub exit: bool,
}

pub struct Count {
    pub modifier: String,
    pub count: u32,
}


impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };

        let source = match args.next() {
            Some(arg) => arg,
            None => String::from(""),
        };

        let count:Count = match args.next() {
            Some(arg) => Count{modifier: arg, count: 4},
            None => {
                if source == "-i" || source == "-h" {
                    Count{modifier: source.clone(), count: 4}
                }
                else {
                    Count{modifier: String::new(), count: 3}
                }

            }         
            
        };

        if count.modifier == "-h" {
            println!("
    This app searches and outputs lines of text file,
    containing the first match to a query.
    Pass args as follows: 
    
    minigrep QUERY FILENAME [i]
    
    OR

    A | minigrep QUERY [i]
    
    Where [i] is a flag for case-insensitive search.
    You can also use ENV variable CASE_INSENSITIVE
    To allow case-insensitive search by default.\n");
        let case_sensitive = false;
        let exit = true;
        Ok(Config {
            query, 
            source, 
            case_sensitive,
            exit,
            })
        }
        
        else {
            if !env::var("CASE_INSENSITIVE").is_err() || count.modifier == "-i" {

                let case_sensitive = false;
                let exit = false;
                Ok(Config {
                    query, 
                    source, 
                    case_sensitive,
                    exit,
                })
            }
            
            else {
                let case_sensitive = true;
                let exit = false;
                Ok(Config {
                    query, 
                    source, 
                    case_sensitive,
                    exit,
                })
            }
        }       
    }
 }


fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> { 
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}


fn get_pipe_output() -> String {
    let mut stdin = io::stdin();
    let mut line = String::new();

    while let Ok(n_bytes) = stdin.read_to_string(&mut line) {
        if n_bytes == 0 { break }
        
    }
    
    line
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