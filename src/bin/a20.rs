// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use::std::io;

enum PowerO{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}
impl PowerO{
    fn new(state: &str) -> Option<PowerO> {
        let state = state.trim().to_lowercase();
        match state.as_str(){
            "off" => Some(PowerO::Off),
            "sleep" => Some(PowerO::Sleep),
            "reboot" => Some(PowerO::Reboot),
            "shutdown" => Some(PowerO::Shutdown),
            "hibernate" => Some(PowerO::Hibernate),
            _ => None

        }
    }
}
// / * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn power(state: PowerO){
    use PowerO::*;
    match state{
    Off => println!("turning off"),
    Sleep => println!("sleeping off"),
    Reboot => println!("rebooting"),
    Shutdown    => println!("shutting down"),
    Hibernate => println!("hibernating"),
    }
}
fn main() {
    // we always have to first create a string baffer when using IO, this will create space for the user input..
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    if user_input_status.is_ok() {
        match PowerO::new(&buffer){
            Some(state) => power(state),
            None => println!("fuck off"),
        }
    }else{
        println!("no such input");
    }
}
