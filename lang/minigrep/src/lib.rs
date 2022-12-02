use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// The Trade-Offs of Using clone
// There’s a tendency among many Rustaceans to avoid using clone to fix ownership problems because of its runtime cost.
// In Chapter 13, you’ll learn how to use more efficient methods in this type of situation.
// As you become more experienced with Rust, it’ll be easier to start with the most efficient solution, but for now, it’s perfectly acceptable to call clone.
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        // We don’t care about the value of the environment variable, just whether it’s set or unset, so we’re checking is_ok
        // rather than using unwrap, expect, or any of the other methods we’ve seen on Result.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// For now, just know that Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
// This gives us flexibility to return error values that may be of different types in different error cases.
// The dyn keyword is short for “dynamic.”
//
// Instead of allowing the program to panic by calling expect, the run function will return a Result<T, E> when something goes wrong.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // we’ve removed the call to expect in favor of the ? operator,
    // Rather than panic! on an error, ? will return the error value from the current function for the caller to handle.
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

// The lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
// In other words, we tell Rust that the data returned by the search function will live
// as long as the data passed into the search function in the contents argument.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines
    // The lines method returns an iterator
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
          results.push(line);
      }
  }

  results
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

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
