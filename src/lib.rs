use std::{ fs, error::Error, env};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_test() {
        let query = "duct";
        let data = "\
Rust:
safe, fast, productive.
pick three
Duct tape
        ";

        assert_eq!(vec!["safe, fast, productive.", "Duct tape"], search(query, data));
    }
}
pub struct Config {
    query: String,
    path: String,
    ignore_case: bool
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments")
        }
        let query = args[1].clone();
        let path = args[2].clone();

        let ignore_case  = env::var("IGNORE_CASE").is_ok();

        Ok(Self { query, path, ignore_case})
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error >> {
    let content = fs::read_to_string(config.path)?;

    let results = if config.ignore_case {
        search_case_insensitive(& config.query, &content)
    } else { search(& config.query, &content) };
    
    for line in results {
        println!{"{line}"};
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results =  Vec::new();
   for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
   }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
   }
    results
}