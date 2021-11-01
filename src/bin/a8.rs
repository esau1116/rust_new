// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor


enum Flavor{
    orange,
    lemon,
    banana
}
struct Drink{
    flavor:Flavor,
    oz:i32,
}
fn my_flav(guess:Drink){
    match guess.flavor{
        Flavor::orange => println!("flavor: orange"),
        Flavor::lemon => println!("flavor: lemon"),
        Flavor::banana => println!("flavor: banana")

    }println!("oz,{:?}", guess.oz);
}

fn main() {
    let first_drink = Drink{
        flavor:Flavor::orange,
        oz:34,
    };
    my_flav(first_drink);
    let second_drink= Drink{
        flavor:Flavor::lemon,
        oz:65,
    };
    my_flav(second_drink);
}
