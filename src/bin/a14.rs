// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: u32,
    name: String,
    fav_color: String,
} 
fn print(data:&str){
    println!("{:?}", data);
}
fn main() {
    let vector = vec![
        Person{
            age:12,
            name:String::from("Paige"),
            fav_color:String::from("blue"),
        },
        Person{
            age:9,
            name:String::from("winnie"),
            fav_color:String::from("orange"),
        },
        Person{
            age:13,
            name:String::from("abe"),
            fav_color:String::from("pink"),
        },
    ];
    for ans in vector{
        if ans.age >= 10{
            print(&ans.name);
            print(&ans.fav_color);
        }
    }
    
}
