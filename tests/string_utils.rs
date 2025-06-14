#[cfg(test)]
mod tests {
    // use super::*;
    // the name of the project is `rust_string_vs_str`
    // so we need to import the function from the crate
    // use the crate name as a prefix
    use rust_string_vs_str::append_world;

    #[test]
    fn test_append_world() {
        let input: &str = "hello";
        let result = append_world(input);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_append_world_empty() {
        let input: &str = "";
        let result = append_world(input);
        assert_eq!(result, " world");
    }
}
