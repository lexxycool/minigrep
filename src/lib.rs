use std::{error::Error, fs}; 

pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(
            Config {
                query,
                file_path,
            }
        )
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(self.config_file_path())?;

        for line in search(&self.query, &contents) {
            println!("{}", line);            
        }
        
        Ok(())
    }

    pub fn config_file_path(&self) -> String {
        self.file_path.to_string()
    }

}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result:Vec<&str> = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
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