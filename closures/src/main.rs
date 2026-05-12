/*
 * Programming in a functional style often includes using functions as values by passing them in arguments,
 * returning them from other functions, assigning them to variables for later execution, and so forth.
 * Closures, a function-like construct you can store in a variable
 * Iterators, a way of processing a series of elements
 */

 /* 
  * Closures are anonymous functions you can save in a variable or pass as arguments to other functions
  * Closures can capture values from the scope in which they're defined, unlike functions
  */

use std::thread;
  
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // closure with no parameters
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


fn add_one_v1 (x: u32) -> u32 {
    x + 1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}  

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

    let _add_one_v2 = |x: u32| -> u32 {
        x + 1
    };
    
    let add_one_v3 = |x| {
        x + 1
    };

    add_one_v3(3); // type of x is now inferred
    
    let add_one_v4 = |x| x + 1;
    add_one_v4(3); // type of x is now inferred


    let example_closure = |x| x;

    // Attempting to call a closure whose types are inferred with two different types:
    let _s = example_closure(String::from("Hello"));
    // let n = example_closure(5);
    // We get a compiler error

    let list = vec![1, 2, 3];
    println!("\nBorrowing immutably:");
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}\n");


    // Mutable reference captured in closure
    println!("Borrowing mutably:");
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {list:?}\n");   

    // `move` keyword
    println!("`move` keyword:");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    // Using move to force the closure for the thread to take ownership of list
    thread::spawn(move || println!("From thread: {list:?}\n")).join().unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    println!("Rectangle list before sort:");
    println!("{list:?}");
    
    list.sort_by_key(|r| r.width);
    println!("Rectangle list after sort:");
    println!("{list:?}");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:?}, sorted in {num_sort_operations} operations");
}

/*
 * FnOnce applies to closures that can be called once. All closures implement at least this trait because all closures can be called.
   A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits because it can only be called once.
   
 * FnMut applies to closures that don’t move captured values out of their body but might mutate the captured values.
   These closures can be called more than once.
   
 * Fn applies to closures that don’t move captured values out of their body and don’t mutate captured values,
  as well as closures that capture nothing from their environment.
  These closures can be called more than once without mutating their environment,
  which is important in cases such as calling a closure multiple times concurrently.
 */

 // Closures are just functions that are defined inline
 // The special thing about them is that they can grab variables from the surrounding scope
 
 // How it grabs them depends on what it does with them:
 // Just reads → borrows immutably &T → Fn
 // Modifies → borrows mutably &mut T → FnMut
 // Takes ownership → moves the value in T → FnOnce, can only call once