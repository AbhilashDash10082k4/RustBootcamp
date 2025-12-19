pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_line () {
        let query = "billionaire";
        let contents = "\
        I will become a billionaire.
        I will build a huge company.";

        assert_eq!(vec!["I", "will", "become", "a", "billionaire"], search(query, contents));
    }
}