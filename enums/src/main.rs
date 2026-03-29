/* Enums allow you to define a type by enumerating its possible variants.
 * enums give you a way of saying a value is one of a possible set of values.
 */

 // enum IpAddressKind {
 //     V4,
 //     V6,
 // }
 
 enum IpAddress {
     V4(String),
     V6(String),
 }
 
 enum Message {
     Quit, // no data associated with it
     Move { x: i32, y: i32 }, // has named fields, like a struct does
     Write(String), // includes a single string
     ChangeColor(i32, i32, i32), // includes three i32 values
 }
 
 // ^^ Equivalent to:
 // struct QuitMessage; // unit struct
 // struct MoveMessage {
 //     x: i32,
 //     y: i32,
 // }
 // struct WriteMessage(String); // tuple struct
 // struct ChangeColorMessage(i32, i32, i32); // tuple struct
 
 // We can define methods for enums using impl like for structs
 impl Message {
     fn call(&self) {
         
     }
 }
 
 // struct IpAddress {
 //     kind: IpAddressKind,
 //     address: String,
 // }
 
 fn route (ip_kind: IpAddress) {
     
 }
 
 // Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.
 // Already available from the standard library (prelude), so need to bring it into scope
 // enum Option<T> {
 //     None,
 //     Some(T),
 // }
 
 #[derive(Debug)]
 enum UsState {
     Alabama,
     Alaska,
     // ... 
 }
 
 impl UsState {
     fn existed_in(&self, year: u16) -> bool {
         match self {
             UsState::Alabama => year >= 1819,
             UsState::Alaska => year >= 1959,
         }
     }
 }
 
 // Checking whether a state existed in 1900 by using conditionals nested inside an if let
 // fn describe_state_quarter(coin: Coin) -> Option<String> {
 //     if let Coin::Quarter(state) = coin {
 //         if state.existed_in(1900) {
 //             Some(format!("{state:?} is pretty old, for America!"))
 //         } else {
 //             Some(format!("{state:?} is relatively new."))
 //         }
 //     } else {
 //         None
 //     }
 // }
 
 // Using if let to produce a value or return early
 // fn describe_state_quarter(coin: Coin) -> Option<String> {
 //     let state = if let Coin::Quarter(state) = coin {
 //         state
 //     } else {
 //         return None;
 //     };
 
 //     if state.existed_in(1900) {
 //         Some(format!("{state:?} is pretty old, for America!"))
 //     } else {
 //         Some(format!("{state:?} is relatively new."))
 //     }
 // }
 
 // Using let...else
 fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else{
         return None;
    };
     
    if state.existed_in(1900){
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
 }
 
 // Using match for pattern matching
 enum Coin {
     Penny,
     Nickel,
     Dime,
     Quarter(UsState),
 }
 
 fn value_in_cents(coin: Coin) -> u8 {
     match coin {
         Coin::Penny => 1,
         Coin::Nickel => 5,
         Coin::Dime => 10,
         Coin::Quarter(state) => {
             println!("Quarter from {:?}!", state);
             25
         }
     }
     // This match expression has 4 arms
 }
 
 
 
 fn plus_one(x: Option<i32>) -> Option<i32> {
     match x {
         None => None,
         Some(i) => Some(i + 1),
     } // match must cover all possibilities
 }

 
fn main() {
    let four = IpAddress::V4;
    let six = IpAddress::V6;
    
    // let home = IpAddress {
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1")
    // };
    
    // let loopback = IpAddress {
    //     kind: IpAddressKind::V6,
    //     address: String::from("::1")
    // };
    
    let home = IpAddress::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddress::V6(String::from("::1"));
    
    let m = Message::Write(String::from("Good morning"));
    m.call();
    
    let some_number = Some(5); // The Compiler infers the type 
    let some_char: Option<char> = Some('g'); // You can also explicitly mention the annotation
    
    let absent_number: Option<i32> = None; // Must mention the type as the compiler cannot infer on an absent value
    // None is similar to 'null'
    // Rust has Option<T> instead of null to not allow for null pointer runtime errors
    // Handling the 'None' case is enforced by the compiler
    
    // You have to convert an Option<T> to a T before you can perform T operations with it.
    // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.
    // Everywhere that a value has a type that isn’t an Option<T>, you can safely assume that the value isn’t null.
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    println!("{:?}\n", five.is_none());
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // catch-all when we want to use the value
    }
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // catch-all when we want don't need to use the value
    }
    
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // nothing happens (empty tuple type)
    }
        
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
    fn reroll() {}
    
    /* if let */
    let config_max = Some(3u8); // 3u8 means 3 of type u8
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
    // Equivalent to:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("No max");
    }
    // if let does not enforce exhaustive checking like match
    // You can also add an else statement to the if let similar to the '_' case in the match statement
    
    
}
