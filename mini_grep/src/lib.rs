use std::fs;
use std::env;
use std::error;

pub struct GrepItems {
    file: String,
    query: String,
    case_sensitive: bool,
}

impl GrepItems {
    pub fn new(mut args: env::Args) -> Result <GrepItems, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not recive a query"),
        };
        let file = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not recive file name"),
        };
        Ok (GrepItems {
            file,
            query,
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(contents: &'a str, query: &str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
    //let mut lines = vec![];
    //let query = query.to_lowercase();
    //for line in contents.split('\n') {
        //if line.to_lowercase().contains(&query) {
            //lines.push(line);
        //}
    //}
    //lines
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
