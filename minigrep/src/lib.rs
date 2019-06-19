use std::env::Args;
use std::fmt::Display;
use std::fs::*;

pub struct Config {
    query: String,
    file_name: String
}

impl From<Vec<String>> for Config {

    fn from(mut l: Vec<String>) -> Config {
        if l.len() < 3 {
            panic!("args length < 3");
        }

        Config {
            query: l.remove(1),
            file_name: l.remove(1)
        }
    }

}

impl Config {

    fn new(l: &[String]) -> Result<Config, &'static str> {
        if l.len() < 3 {
            return Err("args length < 3");
        }

        Ok(Config {
            query: l[1].clone(),
            file_name: l[2].clone()
        })
    }

}

impl Display for Config {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{ query: '{}', file_name: '{}' }}", self.query, self.file_name)
    }

}

pub fn run(args: Args) -> Result<Vec<String>, &'static str> {
    let args: Vec<String> = args.collect();
    /*
    let config: Config = args.into();
    println!("The config:\n\n{}\n", config);
    */

    let config = Config::new(&args)?;

    println!("Search '{}' in file '{}'\n", config.query, config.file_name);

    let content = read_to_string(&config.file_name).expect("Open file failed!");
    println!("The file content: \n\n{}", content);

    let r = grep(config.query, content);
    Ok(r)
}

fn grep(query: String, content: String) -> Vec<String> {
    let lines = content.lines();
    let mut r: Vec<String> = Vec::new();
    for line in lines {
        if line.contains(&query) {
            r.push(String::from(line));
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            grep(query.to_string(), content.to_string())
        );
    }
}