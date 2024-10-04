use std::collections::HashSet;

fn check_all_chars_unique(s: &str) -> bool {
    let set = s.to_lowercase().chars().collect::<HashSet<char>>();
    set.len() == s.len()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert!(check_all_chars_unique("ABCDEFGHIJKLMNOPQRSTUVWXYZ"));
        assert!(!check_all_chars_unique("ABCDEFGHIJKLabc"));
    }
}
