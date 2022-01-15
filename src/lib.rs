use std::env; 
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, 
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); 
        
        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a query string."), 
        }; 

        let filename = match args.next(){
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        }; 

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

// trait 에러 때문에 Box<Error> 가 아니라 Box<dyn Error> 로 반환해야함. 
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results{
        println!("{}", line); 
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect() 
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results 
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
sate, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["sate, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        ); 
    }
}