fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    let name = std::env::args().nth(1).unwrap_or_else(|| "World".to_string());
    println!("{}", greet(&name));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_world() {
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_greet_name() {
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_empty() {
        assert_eq!(greet(""), "Hello, !");
    }
}
