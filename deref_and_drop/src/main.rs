use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref  for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    assert_eq!(*y, 5); // using the dereference operator to access the integer value `y` is pointing to

    let  y = Box::new(x);
    assert_eq!(*y, 5); // using Box<T> like a reference

    // Deref trait
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref()) is what's run behind the scenes

    // Drop trait
    let c = CustomSmartPointer { data: String::from("my stuff"), };
    // c.drop(); // compiler error: "explicit destructor calls not allowed"
    // (it would cause a double free since drop would be called on the value when it goes out of scope)

    // drop(c); // Calling std::mem::drop to explicitly drop a value before it goes out of scope
    
    let d = CustomSmartPointer { data: String::from("other stuff"), };

    println!("CustomSmartPointers created");
}
