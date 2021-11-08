// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

fn main() {
    let mut my_stock = HashMap::new();
        my_stock.insert("chairs", 5);
        my_stock.insert("beds", 3);
        my_stock.insert("Tables", 2);
        my_stock.insert("couches", 0);

        let mut total_count = 0;

        for(item, qty) in my_stock.iter(){
            total_count = total_count + qty;
            let stock_count = if qty == &0{
                "out of stcok".to_owned()
            }else{
                format!("{:?}", qty)
            };
            println!("item={:?}, stock={:?}", item, stock_count);

        }
        println!("total stock{:?}", total_count);
    }


