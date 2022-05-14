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
            &line[colorized_stop..]);
        } 
        

        else {
            let colorized_start = line.to_lowercase().find(&config.query.to_lowercase()).unwrap();
            let colorized_stop = colorized_start + config.query.len();

            println!("{}{}{}", &line[..colorized_start],
            Green.bold().paint(&line[colorized_start..colorized_stop]),
            &line[colorized_stop..]);
            }
        }
        
    Ok(())
}

pub struct Config{
    pub query: String,
    pub filename: String,
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
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let count:Count = match args.next() {
            Some(arg) => Count{modifier: arg, count: 4},
            None => Count{modifier: String::new(), count: 3},

        };

        if count.modifier == "-h" {
            println!("
    This app searches and outputs lines of text file,
    containing the first match to a query.
    Pass args as follows: minigrep.exe query path-to-file [i]
    Where [i] is a flag for case-insensitive search.
    You can also use ENV variable CASE_INSENSITIVE
    To allow case-insensitive search by default.\n");
        let case_sensitive = false;
        let exit = true;
        Ok(Config {
            query, 
            filename, 
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
                    filename, 
                    case_sensitive,
                    exit,
                })
            }
            
            else {
                let case_sensitive = true;
                let exit = false;
                Ok(Config {
                    query, 
                    filename, 
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