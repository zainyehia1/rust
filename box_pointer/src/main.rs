// Boxes are used most often in these situations:
// When you have a type whose size can’t be known at compile time, and you want to use a value of that type in a context that requires an exact size

// When you have a large amount of data, and you want to transfer ownership but ensure that the data won’t be copied when you do so

// When you want to own a value, and you care only that it’s a type that implements a particular trait rather than being of a specific type

// Recursive types
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let b = Box::new(5); // i32 allocated on the heap
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

}
