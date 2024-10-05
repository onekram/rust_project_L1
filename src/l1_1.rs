trait Action {
    fn say(&self);
}

struct Person {
    name: String,
}

impl Action for Person {  // implement trait for struct
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}
impl Person {
    fn new(name: &str) -> Self{ // add constructor
        return Person{name: name.to_string()};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_working() {
        let person = Person::new("Oleg");
        person.say();
    }
}