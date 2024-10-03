fn set_bit(num: i64, idx: usize, value: bool) -> i64 {
    if value {
        num | (1 << idx)
    } else {
        num & !(1 << idx)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn last_bit() {
        assert_eq!(set_bit(8, 0, true), 9);
    }

    #[test]
    fn middle_bit() {
        assert_eq!(set_bit(999, 2, false), 995);
    }
}