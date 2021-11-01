// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

// fn main() {
//     let mut loop1 = 4;
//     loop{
//         println!("{:?}", loop1);
//         loop1 = loop1 - 1;
//         if loop1 == 0 {
//             break;
//         }
//     }
// }
fn main() {
    let mut lot = 8;
    loop{
        println!("{:?}", lot);
        lot = lot - 1;
        if lot == 0 {
            break;
        }
        
    }
}
