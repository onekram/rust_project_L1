fn calculate(a: i64, b: i64) -> (i64, i64, i64, Option<i64>) {
    let sum = a.wrapping_add(b);
    let difference = a.wrapping_sub(b);
    let product = a.wrapping_mul(b);
    let quotient = if b != 0 { Some(a.wrapping_div(b)) } else { None };

    (sum, difference, product, quotient)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        calculate(10000000000, 10000000000);
    }
}
