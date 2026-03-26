use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // no "let" and no "mut" with consts. Also, type of value must be annotated. For example: 'u32'.
    
    // Shadowing
    let y = 5;
    
    println!("\nThe value of y is: {y}");
    
    let y = y + 1;
    
        {
            let y = y * 2;
            println!("The value of y in the inner scope is: {y}");
        }
    
    println!("The value of y is: {y}");
    
    // Shadowing allows changing the type of the value
    let dots = ".....";
    println!("\ndots: {dots}");
    let dots = dots.len();
    println!("dots: {dots}");
    
    /* NOT ALLOWED (COMPILER ERROR) */
    // let mut dots = ".....";
    // dots = dots.len();
    
    
    // Scalar data types
    /* INTEGER, FLOATING-POINT, BOOLEANS, CHARACTERs */
    
    // integer types
    let a: i32 = 98_222; // Decimal
    let b: i32 = 0xff; // Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A'; // Byte (u8 only)
    /* Rust defaults to i32 (32-bit signed integer)
     * When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics.
     * Instead, if overflow occurs, Rust performs two’s complement wrapping. */
    
     // floating-point types
     // Floating-point numbers are represented according to the IEEE-754 standard.
     let o = 2.0; // f64 by default
     let g: f32 = 3.0;
     
     // Boolean
     let t = true;
     
     let f: bool = false; // with explicit type annotation
     
     // Characters
     let z0 = 'z';
     let z: char = 'ℤ'; // with explicit type annotation
     /* Rust’s char type is 4 bytes in size and represents a Unicode scalar value 
      * Accented letters; Chinese, Japanese, and Korean characters; emojis; and zero-width spaces are all valid char values in Rust.
      * Unicode scalar values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. */
     
      // Compound types
      /* TUPLES and ARRAYS */
      
      // tuples
      /* A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
       * Tuples have a fixed length: Once declared, they cannot grow or shrink in size. */
       
       // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
       let tup: (i32, f64, u8) = (500, 6.4, 1);
       
       let (t, u, v) = tup;
   
       println!("The value of u is: {u}");
       
       // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.
       // For example:
       let five_hundred = tup.0;
       
       let six_point_four = tup.1;

       let one = tup.2;
       
       // Arrays
       let arr = [1, 2, 3, 4, 5];
       let arr1: [i32; 5] = [1, 2, 3, 4, 5];
       let arr2 = [3; 5]; // Same as let a = [3, 3, 3, 3, 3];
       /* The array named arr2 will contain 5 elements that will all be set to the value 3 initially. */
       let first = arr[0];
       let second = arr[1];
       
       println!("Please enter an array index.");
       
        let mut index = String::new();
    
        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
    
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
    
        let element = arr[index];
    
        println!("The value of the element at index {index} is: {element}\n");
        
        /* END OF 3.2 */

        /* 3.3 Functions */
        another_function(109);
        print_labeled_measurement(5, 'm');
        
        // Statements & Expressions
        
        // Statements are instructions that perform some action and do not return a value.
        // Ex: let y = 6;, function definitions
        // Expressions evaluate to a resultant value.
        // Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11.
        // Numbers by themselves are also expressions
        // Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:
        let r = {
            let p = 3;
            p + 1
        };
        
        println!("The value of r is: {r}");

        
        /* 3.5 Control flow */
        let number = 3;
    
        // condition must be explicitly a boolean
        if number < 5 {
            println!("first condition was true");
        } else if number > 4 {
            println!("second condition was true")
        } else {
            println!("condition was false");
        }
        
        let number = 6;
        
        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
        
        let condition = 5 < 6;
        let number = if condition { 5 } else { 6 }; // similar to ternary operator "int number = condition ? 5 : 6;"
    
        println!("The value of number is: {number}");

        // Loops
        // loop, while, and for
        // "loop": infinite loop (unless there is break)
        // loop {
        //     println!("again!");
        // }
        
        let mut counter = 0;
    
        let result = loop {
            counter += 1;
    
            if counter == 10 {
                break counter * 2;
            }
        };
    
        println!("The result is {result}");
        
        // Loop labels
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;
    
            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }
    
            count += 1;
        }
        println!("End count = {count}");
        
        // While loop
        let mut number = 3;
    
        while number != 0 {
            println!("{number}!");
    
            number -= 1;
        }
    
        println!("LIFTOFF!!!");
        
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;
    
        while index < 5 {
            println!("the value is: {}", a[index]);
    
            index += 1;
        }
        
        // for loop
        for element in a {
            println!("the value is: {element}");
        }
        
        // rev reverses the range
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
        
        for i in 0..arr.len(){
            print!("{} ", arr[i]);
        }
        print!("\n");
}

// type of parameter must be declared
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions with return values
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon because it's an expression
    // or "return x + 1;"
}