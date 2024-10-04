fn reverse_words(words: &String) -> String {
    words.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        let s = String::from("hello world bye world");
        assert_eq!(reverse_words(&s), "world bye world hello");
    }
}
