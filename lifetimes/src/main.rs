// A lifetime is the scope for which a reference is valid
// The main aim of lifetimes is to prevent dangling references

// Annotations of the lifetimes of r and x, named 'a and 'b, respectively
// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          | Would cause an error because `x` was dropped while still borrowed causing a dangling reference, r.
// }                         // ---------+
// At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.

// No errors:
// fn main() {
//     let x = 5;            // ----------+-- 'b
//                           //           |
//     let r = &x;           // --+-- 'a  |
//                           //   |       |
//     println!("r: {r}");   //   |       |
//                           // --+       |
// }                         // ----------+
// // Here, x lives long enough that r's reference to it is always valid.

// Generic Lifetimes in Functions
// This won't compile:
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() { x } else { y }
// }
// missing lifetime specifier
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`

// Compiles:
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// The returned reference will be valid as long as both of the parameters are valid.
// This is the relationship between lifetimes of the parameters and the return value. 

// Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.
// Once they’re connected, Rust has enough information to allow memory-safe operations and
// disallow operations that would create dangling pointers or otherwise violate memory safety.

// We can define structs to hold references,
// but in that case, we would need to add a lifetime annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

// The lifetime parameter declaration after impl and its use after the type name are required
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
    // There are two input lifetimes,
    // so Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
    // Then, because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    // (Third rule)
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T> (x: &'a str, y: &'a str, ann: T) -> &'a str 
where
    T: Display,
{
    println!("Announcement {ann}");
    if x.len() > y.len() { x } else { y }
}
// This uses all 3 concepts; Generics, Traits, and Lifetimes

fn main(){
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    // Struct holding reference(s)
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    // `novel` doesn’t go out of scope until after the `ImportantExcerpt` goes out of scope,
    // so the reference in the `ImportantExcerpt` instance is valid.

    let s: &'static str = "I have a static lifetime.";
    // 'static denotes that the affected reference can live for the entire duration of the program.
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.


// if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This third rule makes methods much nicer to read and write because fewer symbols are necessary.