// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn main() {
    let mut v = 5;
    while v >= 1{
        println!("{:?}", v);
        v = v - 1;
            if v == 0 {
                println!("done!");
        }
    }
}
