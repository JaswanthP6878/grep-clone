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

        assert_eq!(vec!["safe, fast, productive."], search(query, data));
    }
}
pub struct Config {
    query: String,
    path: String,
    ignore_case: bool
}
impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(val) => val,
            None => return Err("query not given")
        };
        let path = match args.next() {
            Some(val) => val,
            None => return Err("path not mentioned")
        };

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
    contents
        .lines()
        .filter(|value| value.contains(query))
        .collect()
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