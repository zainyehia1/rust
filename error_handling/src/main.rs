use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {
    // panic!() is for unrecoverable errors
    // panic!("crash and burn");
    
    let v = vec![1, 2, 3];

    // v[99]; // would cause the program to panic due to the index being out of bounds
    // The program panics unlike C, where it would be undefined behavior (buffer overread) which is a secuity risk

    // Recoverable Errors
    /* The Result enum is defined as having two variants, Ok and Err, as follows:
     * enum Result<T, E> {
     *   Ok(T),
     *   Err(E),
     * }
     */

     let greeting_file_result = File::open("hello.txt"); // Return type is Result<T, E>
     
     // let greeting_file = match greeting_file_result{
     //     Ok(file) => file, // if result is Ok, the code will return the inner `file` valueout of the Ok variant and assign it to the variable `greeting_file`
     //     Err(error) => panic!("Problem opening file: {error:?}"), // If the result is Err, the program panics and prints the error message.
     // };

     // If file does not exist, create it; else print error message
     let greeting_file = match greeting_file_result{
         Ok(file) => file, 
         Err(error) => match error.kind() {
           ErrorKind::NotFound => match File::create("hello.txt") {
             Ok(fc) => fc,
             Err(e) => panic!("Problem creating the file: {e:?}"),
           },
           _ => {
               panic!("Problem opening the file: {error:?}");
           }
         },
     };

     // same as above without using `match`
     let greeting_file = File::open("hello.txt").unwrap_or_else(|error|{
         if error.kind() == ErrorKind::NotFound {
             File::create("hello.txt").unwrap_or_else(|error|{
                 panic!("Problem creating file: {error:?}");
             })
         } else {
             panic!("Problem opening the file: {error:?}");
         }
     });

     // Easier shortcut (using `unwrap()`)
     let greeting_file = File::open("hello.txt").unwrap();
     // If Ok, unwrap will return the value inside the Ok
     // If Err, unwrap will call panic!() for us.
     
     // Using `expect`
     let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
     /* We use expect in the same way as unwrap: to return the file handle or call the panic! macro.
      * The error message used by expect in its call to panic! will be the parameter that we pass to expect,
      * rather than the default panic! message that unwrap uses.
      */

      // let greeting_file = fs::read_to_string("hello.txt")?; 
      // ^Error The ? operator follows the Result value returned by File::open, but this main function has the return type of (), not Result.
}

// This would allow main to compile with the `?` 
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

/* Propogating Errors */
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e), 
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();

//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();

//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/* The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. */

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}