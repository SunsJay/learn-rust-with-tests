#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        let left = hello("Chris".to_string());
        let right = String::from("Hello, Chris");
        assert_eq!(left, right);
    }
}

fn hello(s: String) -> String {
    format!("Hello, {}", s)
}
