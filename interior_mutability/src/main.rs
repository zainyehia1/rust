// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data;
// normally, this action is disallowed by the borrowing rules.

// To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
// Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us

// The `RefCell<T>` type follows the interior mutability 
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.


// A recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
/* Rc<T> enables multiple owners of the same data;
 * Box<T> and RefCell<T> have single owners. */

/* Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time;
 * RefCell<T> allows immutable or mutable borrows checked at runtime. */
 
/* Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable. */

// Mutating the value inside an immutable value is the interior mutability pattern

use std::{cell::RefCell, rc::Rc};
use crate::List::{Cons, Nil};

fn main() {
    // Immutable values cannot be borrowed mutably as per the borrowing rules:
    // let x = 5;
    // let y = &mut x;

    // `RefCell<T>` is a way to get the ability to have interior mutability
    // The borrow checker in the compiler allows this interior mutability, and the borrowing rules are checked at runtime instead
    // If the rules are violated, you'll get a `panic!` instead of a compiler error.
    
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
    
}

// Allowing multiple owners of mutable data:
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


