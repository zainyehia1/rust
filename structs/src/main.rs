struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username: username,
//         email: email,
//         sign_in_count: 1,
//     }
// }

// Field init Shorthand Syntax
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color (i32, i32, i32);

struct Point (i32, i32, i32);

// Unit-like structs
struct AlwaysEqual; // No fields

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser"),
        email: String::from("someuser@rust.com"),
        sign_in_count: 1
    };
    
    user1.email = String::from("anotheremail@example.com");
    
    // Update syntax, uses a different email from user1 but the rest are the same
    // We can no longer use user1 because the username (heap allocated) from user1 moves to user2 
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Must come last
    };
    
    let user3 = build_user(String::from("random@mail.com"), String::from("random123")); // Using build function
    
    // tuple structs require you to name the type of the struct when you destructure them.
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    
    // let width = 30;
    // let length = 50;
    
    // println!("The area of the rectangle is {} square pixels\n", area(width, length));
    
    // let rectangle1 = (30, 50);
    // println!( "The area of the rectangle is {} square pixels.", area(rectangle1));
    
    let rectangle = Rectangle {width: 30, length: 50};
    println!( "The area of the rectangle is {} square pixels.\n", area(&rectangle));
    
    
    println!("rectangle is {rectangle:?}\n");
    println!("rectangle is {rectangle:#?}\n"); // Better to read
    
    // using the dbg! macro
    
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        length: 50
    };
    
    dbg!(&rect1); // Redirects output to 'stderr'
    
    println!(
        "The area of the rectangle is {} square pixels.\n",
        rect1.area()
    );
    
    let rect1 = Rectangle {
        width: 30,
        length: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        length: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    
    // Use associated function '::'
    let sq = Rectangle::square(3);
}


// fn area (width: u32, length: u32) -> u32 {
//     width * length
// }

// Using tuples
// fn area (dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// Using structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// Methods
impl Rectangle {
    fn area (&self) -> u32 {
        self.width * self.length
    }
    
    fn set_width (&mut self, width: u32){
        self.width = width;
    }
    
    fn can_hold (&self, other_rectangle: &Rectangle) -> bool {
        self.area() > other_rectangle.area()
    }
    
    // All functions defined within an impl block are called associated functions
    // because they’re associated with the type named after the impl.
    
    
    // We can define associated functions that don’t have self as their first parameter (and thus are not methods)
    // because they don’t need an instance of the type to work with.
    
    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
    // For example:
    fn square(size: u32) -> Self { // Self refers to the type that appears after the 'impl' keyword
        Self {
            width: size,
            length: size, 
        }
    }
}
// Each struct can have multiple impl blocks
impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    } // getter
}

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}