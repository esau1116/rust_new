// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Black,
    Blue,
    Yellow,
    Gree
}
fn my_color2(my_color: Color){
    match my_color{
        Color::Black => println!("black"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow"),
        Color::Gree => println!("yep yep yep"),
    }
}
fn main() {
    my_color2(Color::Gree);
    }

