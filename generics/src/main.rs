use core::str;

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
/* Tedious to duplicate this for every type */

// PartialOrd is a trait needed for comparison between items (Similar to the Comparator/Comparable interfaces in Java)
// public <T extends Comparable<T>> T largest(List<T> list) in Java
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics are also used in structs
struct Point<T>{
    x: T,
    y: T,
}

// We have <T> after impl to let the compiler know that T is a generic type parameter and not a concrete type named T
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 

// We can also specify constraints on generic types when defining methods on the type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

/* Using generic types in Rust won’t make programs run any slower than it would with concrete types.
 * Rust accomplishes this by performing monomorphization of the code using generics at compile time.
 * Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
 * The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.
 * It's similar to C++'s templates
 */

 /* Example of Monomorphization:
  * fn main () {
      let integer = Some(5);
      let float = Some(5.0);
   }
   
  * The compiler turns it into this:
  
  * enum Option_i32 {
     Some(i32),
     None,
   }
  
  * enum Option_f64 {
      Some(f64),
      None,
  }
  
  * fn main() {
      let integer = Option_i32::Some(5);
      let float = Option_f64::Some(5.0);
  }
  */

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {largest}");

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {result}");
    
    let point = Point{x: 1, y: 2};
    println!("point.x = {}", point.x());
    let point2 = Point2{x: 5, y: 10.5};
    let p2 = Point2{ x: "Hello", y: 'c' };

    let p3 = point2.mixup(p2);
    
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}