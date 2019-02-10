fn urlify(url: &'static str) -> String {
    let placeholder = "%20";

    url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s)
        } else {
            acc + placeholder + s
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlify() {
        assert_eq!(urlify(&"Mr John Smith    "), "Mr%20John%20Smith");
    }
}

fn main() {
    println!("{}", urlify(&"Mr John Smith    "));
}