fn get_type_of<T>(_: T) -> String{
    let type_name = std::any::type_name::<T>();

    if let Some(pos) = type_name.rfind("::") {
        let trimmed_type = &type_name[pos + 2..];
        return trimmed_type.to_string();
    } else {
        return  type_name.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(get_type_of(&5), "&i32");
        assert_eq!(get_type_of("len"), "&str");
        assert_eq!(get_type_of(vec![1, 2, 3]), "Vec<i32>");
    }
}