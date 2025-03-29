pub fn camel_to_snake_case(camel: String) -> String {
    let mut snake = String::with_capacity(camel.len() + 4);
    let mut chars = camel.chars().peekable();

    while let Some(current) = chars.next() {
        if let Some(&next) = chars.peek() {
            if current.is_ascii_lowercase() && next.is_ascii_uppercase() {
                snake.push(current);
                snake.push('_');
            } else {
                snake.push(current.to_ascii_lowercase());
            }
        } else {
            // Handle the last character
            snake.push(current.to_ascii_lowercase());
        }
    }

    snake
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_to_snake_case() {
        assert_eq!(camel_to_snake_case("camelCase".to_string()), "camel_case");
        assert_eq!(
            camel_to_snake_case("ThisIsATest".to_string()),
            "this_is_a_test"
        );
        assert_eq!(camel_to_snake_case("ABC".to_string()), "a_b_c");
        assert_eq!(camel_to_snake_case("simple".to_string()), "simple");
        assert_eq!(camel_to_snake_case("".to_string()), "");
    }
}
