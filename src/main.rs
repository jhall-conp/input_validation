use cs50_input::{get_input, get_bool, get_choice, get_list};

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

    let numbers: Vec<String> = get_list("Enter some names, separated by commas: ", ",");
    println!("You entered: {:?}", numbers);
    
    let numbers: Vec<i64> = get_list("Enter some numbers, separated by commas: ", ",");
    println!("You entered: {:?}", numbers);
}
