// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
struct box_char{
    dimension:Dimensions,
    weight: i32,
    color; Color,
}
// * Use an enum for the box color
enum Color{
    blue,
    red
}
// * Implement functionality on the box struct to create a new box
impl{
    fn box(&self){
        println!("{:?} new box", self.Co)
    }
}
// * Implement functionality on the box struct to print the characteristics

fn main() {}
