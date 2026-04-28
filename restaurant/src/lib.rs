// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist(){
            
//         }
        
//         // fn seat_at_table(){
            
//         // }
//     }
    
//     // mod serving {
//     //     fn take_order() {
            
//     //     }
        
//     //     fn serve_order() {
            
//     //     }
        
//     //     fn take_payment() {
            
//     //     }
//     // }
// }

// pub fn eat_at_restuarant () {
//     // Absolute Path
//     crate::front_of_house::hosting::add_to_waitlist();
    
//     // Relative Path
//     front_of_house::hosting::add_to_waitlist();
// }

/* The "super" keyword */
// fn serve_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order(); // instead of crate::serve_order();
//     }
//     fn cook_order() {
        
//     }
// }


/* members of structs are private by default */
// mod back_of_house {
//     pub struct Breakfast{
//         pub toast: String,
//         seasonal_fruit: String,
//     }
    
//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast{
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches")
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = back_of_house::Breakfast::summer("Rye");
    
//     meal.toast = String::from("Wheat");
// }


/* Variants of enums are public by default */
// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant(){
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

/* The "use" keyword */
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
            
//         }
//     }
// }

// Bringing hosting into scope
// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
/* The idiomatic way in Rust to bring a function into scope is to bring the function's parent module into scope */

// pub fn eat_at_restaurant(){
//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

/* If you're bringing in enums, structs, or others into scope then it is idiomatic to specify the full path unless two things have the same name */
// use std::fmt;
// use std::io;

// fn func1() -> fmt::Result {
    
// }

// fn func2() -> io::Result<()> {
    
// }

/*
 * Another option is to rename one of the result types
 * "as" keyword
 */
// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn func1() -> Result {
//     Ok(())
// }

// fn func2() -> IoResult<()> {
//     Ok(())
// }


/* Letting external code have access */
// use rand::Rng;
// use rand::CryptoRng;
// use rand::Fill;
use rand::{Rng, CryptoRng, Fill}; // nested paths

// use std::io;
// use std::io::Write;
use std::io::{self, Write};
use std::io::*; // Now all public items under std::io are in scope

/* Seperating Modules into different files */
mod front_of_house;

pub use crate::front_of_house::hosting; // External code can reference 'hosting' due to the "pub" keyword here

pub fn eat_at_restaurant(){
    let secret_number = rand::thread_rng().gen_range(1..101);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}