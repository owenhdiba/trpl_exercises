use std::error::Error;
use std::fs;

/// Holds the settings passed in via CLI
pub struct Config {
    /// The phrase to search for.
    pub query: String,
    /// The file to search in.
    pub file_path: String,
    /// Use case-insensitive search.
    pub ignore_case: bool,
}

impl Config {
    /// Try to build `Config` from command line args.
    /// If there is no argument passed for `ignore_case` (3rd argument),
    /// will default to false.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = match args.next().unwrap_or("false".to_string()).parse::<bool>() {
            Ok(ignore_case) => ignore_case,
            Err(_) => {
                return Err(
                    "Could not interpret `ignore_case` argument. Argument should be `true` or `false`.",
                );
            }
        };

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Read in text file `config.file_path` and search for lines
/// that contain the substring `config.query`. If `config.ignore_case == false`
/// (default) then performs case sensitive search, otherwise runs a case insensitive
/// search.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

/// Search for lines in `contents` that contain `query`. Case sensitive.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|l| l.contains(query)).collect()
}

/// Search for lines in `contents` that contain `query`. Case insensitive.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_success() {
        let config = Config::build(
            vec![
                "rust_file_name".to_string(),
                "query".to_string(),
                "file.txt".to_string(),
            ]
            .into_iter(),
        );
        assert!(matches!(
            config,
            Ok(Config {
                query: _,
                file_path: _,
                ignore_case: _
            })
        ));
    }

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
    fn case_insensitive() {
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
