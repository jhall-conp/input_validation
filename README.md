﻿[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Input Universal Input Validation

Prompts the user for input and parses to the value of the type requested. If an incorrect response is provided the user will be prompted again until they give a proper response.

## Installation

To use this crate in your project, either add `input_validation` your `Cargo.toml` file or run 'cargo add input_validation`.

## Usage

```rust
use input_validation::{get_input,get_bool,get_list,get_choice, get_email};

fn main() {
    let name: String = get_input("What is your name? ");
    println!("Hello, {}", name);

    let age: u8 = get_input("What is your age? ");
    println!("Your are {} years old.", age);

    let is_cool: bool = get_bool("Are you cool? ");
    println!("You are{} cool!", if is_cool { ""} else {" not"});

    let choices = &["Apples", "Oranges", "Bananas"];
    let favorite_fruit = get_choice("What is your favorite fruit? ",choices);
    println!("Your favorite fruit is: {}", choices[favorite_fruit]);

    let names: Vec<String> = get_list("Enter some names, separated by commas: ", ",");
    println!("You entered: {:?}", names);

    let numbers: Vec<i64> = get_list("Enter some numbers, separated by commas: ", ",");
    println!("You entered: {:?}", numbers);

    let email = get_email("Enter your email address: ");
    println!("Your email address is: {}", email);
}
```

### Example Output
```rust
What is your name? 4
Hello, 4
What is your age? John Smith
What is your age? 43
Your are 43 years old.
Are you cool? k
Are you cool? Y
You are cool!
What is your favorite fruit?  (Apples/Oranges/Bananas) apple
What is your favorite fruit?  (Apples/Oranges/Bananas) apples
Your favorite fruit is: Apples
Enter some names, separated by commas: John Cougar, Fred Flintstone, Mary Cooper, violet beauregarde 
You entered: ["John Cougar", "Fred Flintstone", "Mary Cooper", "violet beauregarde"]
Enter some numbers, separated by commas: j, k, w, 3
Enter some numbers, separated by commas: 1, 5, 3, 98, 42
You entered: [1, 5, 3, 98, 42]
Enter your email address: john@example.com
Your email address is: john@example.com
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

## License

[MIT](https://opensource.org/licenses/MIT)