/// Tests must be run individually, not with `cargo test`.
/// Respond with the text displayed on the screen or within the parentheses.
use crate::{get_input, get_list, get_bool, get_choice, get_email, get_separator};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_separator() {
        let separator = get_separator();
        assert_eq!(separator, '.');
    }

    #[test]
    fn test_get_input_string() {
        let input = "Enter: Hello, world!: ";
        let result: String = get_input(input);
        assert_eq!(result, "Hello, world!");
    }

    #[test]
    fn test_get_input_i32() {
        let input = "Enter 42: ";
        let result: i32 = get_input(input);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_get_input_i32_comma() {
        let input = "Enter 1,454,398: ";
        let result: i32 = get_input(input);
        assert_eq!(result, 1454398);
    }

    #[test]
    fn test_get_input_float() {
        let input = "Enter 3.14: ";
        let result: f32 = get_input(input);
        assert_eq!(result, 3.14);
    }

    #[test]
    fn test_get_input_float_comma() {
        let input = "Enter 1,000.34: ";
        let result: f32 = get_input(input);
        assert_eq!(result, 1000.34);
    }

    #[test]
    fn test_get_list() {
        let prompt = "Enter comma-separated values: 1, 2, 3: ";
        let separator = ",";
        let result: Vec<i32> = get_list(prompt, separator);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_get_bool() {
        let input = "Enter n: ";
        let result: bool = get_bool(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_get_choice() {
        let prompt = "Choose a color (blue): ";
        let choices = &["red", "green", "blue"];
        let result: usize = get_choice(prompt, choices);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_get_email() {
        let prompt = "Enter your email address - john@example.com: ";
        let result: String = get_email(prompt);
        assert_eq!(result, "john@example.com");
    }
}
