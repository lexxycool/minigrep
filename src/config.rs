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
    
    pub fn config_query(&self) -> String {
        self.query.to_string()
    }

    pub fn config_file_path(&self) -> String {
        self.file_path.to_string()
    }
}