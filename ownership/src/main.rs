fn main() {
    // ------- Ownership Rules --------
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    
    { // s is not valid here, since it's not yet declared
        let _s = "hello";   // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid
    // Rust calls a function called "drop" automatically at the closing curly bracket.
    // Inspired by C++'s RAII (Resource Acquisition Is Initialization)
    // Which is the pattern of deallocating resources at the end of an item's lifetime
    
    // heap allocated strings
    let s1 = String::from("hello");
    let _s2 = s1.clone(); // deep copy (explicit). No silent copies, no double frees.
    let _s3: String = s1; // Move (not shallow copy), s1 is now invalid, like std::move() in C++. If it were to shallow copy, it would double free when the variables go out of scope.
    // If you try to print s1, you get a compiler error. In C++, it would cause undefined behaviour but compiles

    
    // Can be mutated, unlike string literals
    let mut s = String::from("hello");
    
    s.push_str(", world"); // push_str() appends a literal to a String
    
    println!("{s}\n");
    
    // No move here since they are allocated on the stack and not the heap
    let x = 5;
    let y = x; // Copy trait
    
    // Integer types, boolean types, floating-point types, characters all implement Copy
    // Tuples implement types ONLY if they only contain types that also implement the Copy trait
    
    println!("x: {x}, y: {y}\n");
    
    // When you assign a completely new value to an existing variable, Rust will call drop and free the original value’s memory immediately.
    let mut greet = String::from("hello");
    greet = String::from("ahoy");
    
    println!("{greet}, world!\n"); // prints "ahoy, world!"
    
    
    // Passing a variable to a function will move or copy, just as assignment does.
    let s = String::from("hello");  // s comes into scope
 
     takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    // To not have this happen, we need borrowing.
 
     let x = 5; // x comes into scope
 
     makes_copy(x);  // Because i32 implements the Copy trait,
                    // x does NOT move into the function,
                   // so it's okay to use x afterward.
 
    // At the end of the main func, x goes out of scope, then s. However, because s's value was moved, nothing special happens.
    
    // Returning values can also transfer ownership
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3     
                                     
    println!("s1: {s1}, s3: {s3}\n"); // attempting to access s2 would cause a compile-time error due to it being moved to s3
    
    let multiple = String::from("hello");

    let (m1, len) = calculate_length(multiple);

    println!("The length of '{m1}' is {len}.");
    
    let r = String::from("rust");
    
    let len = calculate_length_r(&r); // Borrows the value of r and doesn't move it
    
    println!("The length of '{r}' is {len}.\n");
    
    // You CANNOT modify a borrowed value unless it's a mutable reference
    // References are immutable by default
    
    // Mutable references
    let mut ss1 = String::from("hello");
    println!("ss1's value before modifying: {ss1}");

    change(&mut ss1);
    println!("ss1's value after modifying: {s1}\n");
    
    // If you have a mutable reference to a value, you CANNOT have other references to that value.
    // You can have many immutable references OR one mutable reference
    
    {
        let _r1 = &mut ss1;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    
    let _r2 = &mut ss1;
    
    // NOTE that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
    let r3 = &ss1; // no problem
    let r4 = &ss1; // no problem
    println!("{r3} and {r4}");
    // Variables r1 and r2 will not be used after this point.
    
    let r5 = &mut ss1; // no problem
    println!("{r5}");
    // The scopes of the immutable references r1 and r2 end after the println! where they are last used,
    // which is before the mutable reference r3 is created.
 
    
    /* 4.3 Slice Type */
    let mut test = String::from("hello world");

    let _word = first_word(&test); // word will get the value 5

    test.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but s no longer has any content that we
    // could meaningfully use with the value 5, so word is now totally invalid!
    
    // Solution: String slices
    test = String::from("hello world");
    let hello = &test[0..5];
    let world = &test[6..11];
    
    // Equal:
    let slice = &test[0..2];
    let slice = &test[..2];
    
    let len = test.len();
    
    let slice = &test[3..len];
    let slice = &test[3..];
    
    let slice = &test[0..len]; 
    let slice = &test[..];
    
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole.
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    
    // Slicing arrays
    let a = [1, 2, 3, 4, 5];
    
    let slice = &a[1..3];
    
 } 
   // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
   // happens. s1 goes out of scope and is dropped.
 
 fn takes_ownership(some_string: String) { // some_string comes into scope
     println!("{some_string}");
 } // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
 
 fn makes_copy(some_integer: i32) { // some_integer comes into scope
     println!("{some_integer}");
 } // Here, some_integer goes out of scope. Nothing special happens.
 
 fn gives_ownership() -> String { // gives_ownership will move its return value into the function that calls it
 
     let some_string = String::from("yours"); // some_string comes into scope
 
     some_string // some_string is returned and moves out to the calling function
 }
 
 // This function takes a String and returns a String.
 fn takes_and_gives_back(a_string: String) -> String {
     // a_string comes into
     // scope
 
     a_string  // a_string is returned and moves out to the calling function
 }
 
 // Rust does let us return multiple values using a tuple
 fn calculate_length(s: String) -> (String, usize) {
     let length = s.len(); // len() returns the length of a String
 
     (s, length)
 }
 
 // Calculate length with reference to a String
 fn calculate_length_r(s: &String) -> usize { // s is a reference to a String
     s.len()
 } // Here, s goes out of scope. But because s does not have ownership of what it refers to, the String is not dropped.
  // References allow you to refer to some value without taking ownership of it.
  // The action of creating references is called borrowing
  
  // Modifying mutable references
  fn change(some_string: &mut String) {
      some_string.push_str(", world");
  }
  
/* 
  // Dangling references
  fn dangle() -> &String { // dangle returns a reference to a String
  
      let s = String::from("hello"); // s is a new String
  
      &s // we return a reference to the String, s
  } // Here, s goes out of scope and is dropped, so its memory goes away.
    // Danger!
*/

// String Slicing
// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// Rewrite the function to return a slice
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
