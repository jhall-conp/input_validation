//! Prompts the user for input and parses the input as a value of type 'T'.
//!
//! If the input cannot be parsed as the request 'T', the function will prompt the user again
//! until a valid input is entered.
//! 
//! # Arguments
//! 
//! * 'prompt' - A string that will be printed to the console to prompt the user for input.
//! 
//! # Returns
//! 
//! A value of type 'T' that was parsed from the user's input.
//! 
//! # Panics
//! This function will panic if 'T' does not implement the 'FromStr' trait.
use std::io::{self, Write};
use regex::Regex;

/// Prompts the user for input and parses the user's response as a specified type.
///
/// This function repeatedly prompts the user for input until the user provides input that can be
/// successfully parsed as the specified type `T`. If the user's input cannot be parsed as `T`,
/// the function will continue to prompt the user for input.
///
/// # Examples
///
/// ```
/// use input_validation::get_input;
///
/// let name: String = get_input("What is your name? ");
/// println!("Hello, {}!", name);
/// ```
///
/// ```
/// use input_validation::get_input;
///
/// let age: u32 = get_input("How old are you? ");
/// println!("You are {} years old.", age);
/// ```
///
/// # Panics
///
/// This function will panic if it is unable to write to the standard output stream.
///
/// # Errors
///
/// This function will return an error if it is unable to read from the standard input stream or
/// if the user's input cannot be parsed as the specified type `T`.
pub fn get_input<T: std::str::FromStr>(prompt: &str) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim().parse::<T>() {
                Ok(val) => return val,
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }
}

/// Reads input from the user and returns a vector of elements of type T.
///
/// This function prompts the user for input using the specified `prompt` and then splits
/// the input string into individual elements using the specified `separator`. If all of the
/// elements can be parsed into type `T`, then they are returned as a `Vec<T>`. If any of the
/// elements fail to parse, the function will loop and prompt the user for input again.
///
/// # Examples
///
/// ```rust
/// let numbers: Vec<i32> = get_list("Enter some numbers, separated by commas: ", ",");
/// ```
///
/// # Panics
///
/// This function will panic if `separator` is an empty string.
///
/// # Errors
///
/// This function will keep looping and prompting for input if any of the elements fail to parse
/// into type `T`. The error message from the parsing failure is printed to stderr.
///
pub fn get_list<T: std::str::FromStr>(prompt: &str, separator: &str) -> Vec<T>
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    loop {
        let input = get_input::<String>(prompt);
        let values: Vec<_> = input.split(separator).map(str::trim).collect();
        if values.iter().all(|s| s.parse::<T>().is_ok()) {
            return values.iter().map(|s| s.parse::<T>().unwrap()).collect();
        }
    }
}

/// Prompts the user for a boolean input, returning `true` if the input is
/// "y" or "yes" (case insensitive), and `false` if the input is "n" or "no"
/// (case insensitive). If the input does not match either "y", "yes", "n", or
/// "no", the function will loop and prompt the user again.
///
/// # Arguments
///
/// * `prompt` - A string slice that will be displayed to the user as the prompt
/// for their input.
///
/// # Example
///
/// ```
/// use user_input::{get_bool};
///
/// let confirm = get_bool("Are you sure you want to proceed? (y/n) ");
/// if confirm {
///     println!("User confirmed.");
/// } else {
///     println!("User declined.");
/// }
/// ```
pub fn get_bool(prompt: &str) -> bool {
    loop {
        let input = get_input::<String>(prompt).to_lowercase();
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => continue,
        }
    }
}

/// Prompts the user to select a choice from a list of options and returns the index of the selected choice.
///
/// # Arguments
///
/// * `prompt` - A string slice containing the prompt to display to the user.
/// * `choices` - A slice containing the available choices.
///
/// # Examples
///
/// ```
/// let choices = vec!["Option 1", "Option 2", "Option 3"];
/// let index = get_choice("Please select an option", &choices);
/// println!("You selected: {}", choices[index]);
/// ```
    pub fn get_choice(prompt: &str, choices: &[&str]) -> usize {
    loop {
        let choice = get_input::<String>(&format!("{} ({}) ", prompt, choices.join("/")));
        match choices
            .iter()
            .position(|&c| c.to_lowercase() == choice.to_lowercase().trim())
        {
            Some(index) => return index,
            None => continue,
        }
    }
}


/// Prompts the user to enter an email address and returns it as a string.
///
/// The function ensures that the email address is valid and has the correct format.
///
/// # Examples
///
/// ```
/// use rust_input_lib::get_email;
///
/// let email = get_email("Enter your email address: ");
/// println!("Your email address is: {}", email);
/// ```
pub fn get_email(prompt: &str) -> String {
    let re = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    loop {
        let email = get_input::<String>(prompt);
        if re.is_match(&email) {
            return email;
        } else {
            println!("Invalid email address. Please enter a valid email address.");
            continue;
        }
    }
}

#[cfg(test)]
mod integration_tests;
