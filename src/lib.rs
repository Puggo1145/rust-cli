pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_each<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(query))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_one_by_one() {
        let query = "hello";
        let contents = "\
Rust:
hello world.
hello Andrew.
hello me";

        let mut search_iter = search_each(query, contents); 
        let res1 = match search_iter.next() {
            Some(res) => res,
            None => "X"
        };
        let res2 = match search_iter.next() {
            Some(res) => res,
            None => "X"
        };
        let res3 = match search_iter.next() {
            Some(res) => res,
            None => "X"
        };

        assert_eq!("hello world.", res1);
        assert_eq!("hello Andrew.", res2);
        assert_eq!("hello me", res3);
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
