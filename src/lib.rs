pub fn append_world(input: &str) -> String {
    let mut result = String::from(input);
    result.push_str(" world");
    result
}
