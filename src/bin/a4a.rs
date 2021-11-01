// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn main() {
    let my_name2 = "duck";
    match my_name2 {
        "kaks" => println! ("its false"),
        "duck" => println! ("its true"),
        _ => println! ("what is it"),
    }
}
