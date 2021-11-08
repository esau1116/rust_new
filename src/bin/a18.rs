// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase
struct Customer{
    age:i32,
}
fn try_purchase(customer: &Customer) -> Result<(), String>{
    if customer.age < 21{
        Err("customer is below age".to_owned())
    }else{
        Ok(())
    }
}
fn main() {
    let ashley = Customer{age:20};
    let purchase = try_purchase(&ashley);
    println!("{:?}", purchase);
}
