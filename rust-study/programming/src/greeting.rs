pub fn greeting(name: &str) -> String {
    format!("{}", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "{}", result);
    }
}
