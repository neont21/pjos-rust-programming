pub fn greeting(name: &str) -> String {
    String::from("Hello?")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Peter");
        assert!(result.contains("Peter"),
            "Greeting did not contain name, value was '{}'",
            result
        );
    }
}
