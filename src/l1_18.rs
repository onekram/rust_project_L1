fn reverse_strings(s: &String) -> String {
    s.chars().rev().collect::<String>()  // Split string to chars (one char for one unicode character) and collect to string
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_strings() {
        let s = String::from("Привет");
        assert_eq!(reverse_strings(&s), "тевирП");
    }
}
