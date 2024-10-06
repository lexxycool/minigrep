use std::{env, error::Error, fs}; 

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool, // Another option is not to use this field
                      // and have search function do both the 
                     // case-sensitive and case-insensitive
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(
            Config {
                query,
                file_path,
                ignore_case // set the env variable in the terminal 
                            //eg  $Env:IGNORE_CASE=1; cargo run -- RUSt poem.txt
                            //unset it, use Remove-Item Env:IGNORE_CASE
            }
        )
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.config_file_path())?;

        let results = match self.ignore_case {
            true => search_case_sensitive(&self.query, &contents),
            false => search(&self.query, &contents),          
        };

        for line in results {
            println!("{}", line);            
        }
        
        Ok(())
    }

    pub fn config_file_path(&self) -> String {
        self.file_path.to_string()
    }

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result:Vec<&str> = Vec::new();
    
    // for line in contents.lines() {
    //     if line.contains(query) { // another option for both case- sensitive 
    //                              // and case-insensitive (no need for search_case_sensitive function)
    //                             // line.to_lowercase().contains(&query.to_lowercase()
    //         result.push(line);
    //     }
    // }
    // result
    contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result:Vec<&str> = Vec::new();
    
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         result.push(line);
    //     }
    // }
    // result
    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
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

    #[test]
    fn case_sensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_sensitive(query, contents)); 
        // can have search function here when it does both case-sensitive and case-insensitive.
    }

}