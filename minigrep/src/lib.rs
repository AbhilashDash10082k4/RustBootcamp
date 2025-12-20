pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //contents is related to the return type as the return val contains the string ref from content param
    // unimplemented!(); -this panics on a test run
    /*lines() -> line by line iterator */
    let mut store = Vec::new();
    for line in contents.lines() {
        // line containing the query
        if line.contains(query) {
            store.push(line);
        }
    }
    store
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // to_lowercase() creates a new string and does not references the existing data
    let query = query.to_lowercase();
    let mut store = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            store.push(line);
        }
    }
    store
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "billionaire";
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
