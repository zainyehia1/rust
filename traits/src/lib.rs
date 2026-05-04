use std::{fmt::{Debug, Display}};

pub trait Summary {
    fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        // String::from("(Read more...)") // Default implementation
        format!("(Read more from {}...)", self.summarize_author()) // default impelemtntation calls another method in the same trait
    }
    // It isn’t possible to call the default implementation from an overriding implementation of that same method.
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("by: {}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


// Using Traits as Parameters
// This parameter accepts any type that implements the specified trait.
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize())
// }
// ^^^ Is "syntax sugar" for a longer form known as a trait bound:
/* pub fn notify<T: Summary>(item: &T){
 *     println!("Breaking news! {}", item.summarize())
 * } 
 */

// pub fn notify(item1: &impl Summary, item2: &impl Summary){}
// ^^^ allows `item1` and `item2` to have different types as long as both implement Summary

// pub fn notify<T: Summary>(item1: &T, item2: &T){}
// ^^^ forces both parameters to have the same type.
//     The generic type T specified as the type of the item1 and item2 parameters constrains the function
//     such that the concrete type of the value passed as an argument for item1 and item2 must be the same.

// Multiple Trait Bounds with the `+` Syntax
pub fn notify(item: &(impl Summary + Display)){
    println!("Breaking news! {}", item.summarize())
}

// The + syntax is also valid with trait bounds on generic types:
// pub fn notify<T: Summary + Display>(item: &T){}

// Clearer Trait Bounds with where Clauses
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// ^^^ Difficult to read, use alternate syntax using the `where` keyword:
fn some_function<T,U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    1
}

// Returning Types That Implement Traits
// We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait,
// as shown here:
fn returns_summarizable() -> impl Summary {
    SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        repost: false,
    }
} // Only works if you're returning a single type


// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Here Pair<T> only implements the cmp_display method if
// its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self){
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}