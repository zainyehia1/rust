// Rc<T> is only for use in single-threaded scenarios.
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
    // `Rc::clone` doesn't make a deep copy of all the data like most types' implementations of `clone` do.
    // The call to `Rc::clone` only increments the reference count, which doesn't take much time unlike deep copies

    // Box<T> is like the unique_ptr in C++
    // `Rc::clone` is like the shared_ptr in C++
    // `Rc::downgrade is like the weak_ptr in C++
    
    // Cloning to Increase the Reference Count
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // The implementation of the Drop trait decreases the reference count automatically when an Rc<T> value goes out of scope.
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    // Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.
    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
}