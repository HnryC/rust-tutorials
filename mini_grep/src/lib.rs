use std::fs;
use std::env;
use std::error;

pub struct GrepItems {
    file: String,
    query: String,
    case_sensitive: bool,
}

impl GrepItems {
    pub fn new(args: &[String]) -> Result <GrepItems, &str> {
        if args.len() < 3 {
            return Err("Not enough supplied arguments");
        }
        Ok (GrepItems {
            file: args[1].clone(),
            query: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

pub fn run(items: &GrepItems) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(&items.file)?;
    let results = if items.case_sensitive {
        search(&contents, &items.query)
    } else {
        search_case_insensitive(&contents, &items.query)
    };
    if results.len() == 0 {
        println!("{}", results.len());
    }
    for result in results {
        println!("{}", result);
    }
    Ok(())
}

pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str>{
    let mut lines = vec![];
    for line in contents.split('\n') {
        if line.contains(query) {
            lines.push(line);
        }
    }
    lines
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str>{
    let mut lines = vec![];
    let query = query.to_lowercase();
    for line in contents.split('\n') {
        if line.to_lowercase().contains(&query) {
            lines.push(line);
        }
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_line_search() {
        let contents = "Hello";
        let query = "ell";
        assert_eq!(vec!["Hello"], search(contents, query));
    }

    #[test]
    fn mulit_line_search() {
        let contents = "Hello
How are you?
Go to hell
Did you know that sherlock never said 'elementry dear watson' in the sherlock books";
        let query = "el";
        assert_eq!(vec!["Hello", "Go to hell", "Did you know that sherlock never said 'elementry dear watson' in the sherlock books"], search(contents, query));
    }

    #[test]
    fn case_insensitive_search() {
        let contents = "Hello
How are you?
Go to hell
Did you know that sherlock never said 'elementry dear watson' in the sherlock books
Elements are the basis for all matter";
        let query = "el";
        assert_eq!(vec!["Hello", "Go to hell", "Did you know that sherlock never said 'elementry dear watson' in the sherlock books","Elements are the basis for all matter"], search_case_insensitive(contents, query));
    }
}
