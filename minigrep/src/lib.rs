use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new<'a, T>(mut args: T, env_vars: HashMap<String, String>) -> Result<Config, &'a str>
    where
        T: Iterator<Item = String>,
    {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        Ok(Config {
            query,
            filename,
            case_sensitive: !env_vars.contains_key("CASE_INSENSITIVE"),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    println!("With contents:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Found lines:");
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_new_takes_second_and_third_args_and_sets_case_sensitive_with_no_env_vars() {
        let args = vec![
            String::from("skipped"),
            String::from("test"),
            String::from("test.txt"),
        ]
        .into_iter();
        let env_vars: HashMap<String, String> = HashMap::new();

        let config = Config::new(args, env_vars).unwrap();

        assert_eq!(config.query, String::from("test"));
        assert_eq!(config.filename, String::from("test.txt"));
        assert_eq!(config.case_sensitive, true);
    }

    #[test]
    fn config_new_takes_second_and_third_args_and_sets_case_insensitive_with_flag() {
        let args = vec![
            String::from("skipped"),
            String::from("test"),
            String::from("test.txt"),
        ]
        .into_iter();
        let env_vars = vec![(String::from("CASE_INSENSITIVE"), String::from(""))]
            .into_iter()
            .collect();

        let config = Config::new(args, env_vars).unwrap();

        assert_eq!(config.query, String::from("test"));
        assert_eq!(config.filename, String::from("test.txt"));
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    fn config_new_returns_err_result_when_no_second_arg() {
        let args = vec![String::from("first")].into_iter();
        let env_vars: HashMap<String, String> = HashMap::new();

        match Config::new(args, env_vars) {
            Ok(_) => panic!("Err expected"),
            Err(msg) => assert_eq!(msg, "Didn't get a query string"),
        }
    }

    #[test]
    fn config_new_returns_err_result_when_no_third_arg() {
        let args = vec![String::from("first"), String::from("second")].into_iter();
        let env_vars: HashMap<String, String> = HashMap::new();

        match Config::new(args, env_vars) {
            Ok(_) => panic!("Err expected"),
            Err(msg) => assert_eq!(msg, "Didn't get a file name"),
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nDuct tape.\n";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\nRust:\nsafe, fast, productive.\nPick three.\nTrust me.\n";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
