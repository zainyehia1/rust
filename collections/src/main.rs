fn main() {
    /* Vectors */
    // let mut v: Vec<i32> = Vec::new();
    
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    
    // Reading Vector Elements
    let v = vec![1, 2, 3, 4, 5]; // infers type
    let third: &i32 = &v[2];
    
    println!("The third element in v is {third}");
    
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element in v is {third}"),
        None => println!("There is no third element in v."),
    }

    // let _does_not_exist = &v[100];
    // println!("The 100th element in v is {_does_not_exist}"); // Program will panic
    let _does_not_exist = v.get(100); // Would not panic ('None')
    
    
    /* Attempting to add an element to a vector while holding a reference to an item */
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // immutable borrow

    // v.push(6); // cannot borrow v as mutable because it is also borrowed as immutable

    // println!("The first element is: {first}");

    /*
     * This error is due to the way vectors work:
     * Because vectors put the values next to each other in memory,
     * adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space,
     * if there isn’t enough room to put all the elements next to each other where the vector is currently stored.
     * In that case, the reference to the first element would be pointing to deallocated memory.
     * The borrowing rules prevent programs from ending up in that situation.
    */
    
    /* Iterating over values in a vector */
     
     // Non-mutable vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }
    println!();
     
     // Mutable vector
     // Iterating and adding 50 to each element
    let mut v = vec![100, 32, 57];
    for i in &mut v {
         *i += 50; // dereference i using *i, then add 50 to it
         println!("{i}");
    }

     /* Using an Enum to store multiple types */
     enum SpreadSheetCell {
         Int(i32),
         Float(f64),
         Text(String),
     }
     
     let row = vec![
         SpreadSheetCell::Int(3),
         SpreadSheetCell::Text(String::from("blue")),
         SpreadSheetCell::Float(10.12),
     ];
     
     // Rust needs to know what types will be in the vector at compile time so that it knows exactly
     // how much memory on the heap will be needed to store each element.
     
     /* Strings */
     let mut s = String::new();
     
     // to_string method
     let data = "initial contents";
     let s = data.to_string();
     
     let s = "initial contents".to_string();
     
     // String::from()
     let s = String::from("initial contents");
     
     // Strings are UTF-8 encoded
     let salam = String::from("السلام عليكم");
     println!("\n{salam}\n");
     
     // String concatenation
     let mut s = String::from("foo");
     s.push_str("bar");
     s.push('r'); // push() is for chars
     
     let mut  s1 = String::from("foo");
     let s2 = "bar";
     s1.push_str(s2);
     println!("s2 is {s2}");
     
     // Concatenating using '+'
     let s1 = String::from("Hello, ");
     let s2 = String::from("world!");
     let s3 = s1 + &s2; // `+` cannot be used to concatenate two `&str` strings, string concatenation requires an owned `String` on the left
     // s1 is moved to s3 while s2 is borrowed
     /* 
      * String concatenation appends the string on the right to the string on the left and may require reallocation.
      * This requires ownership of the string on the left.
      */
      
      let s1 = String::from("tic");
      let s2 = String::from("tac");
      let s3 = String::from("toe");
  
      let s = s1 + "-" + &s2 + "-" + &s3;
      let s1 = String::from("tic");
      let s = format!("{s1}-{s2}-{s3}"); // works like the `println!` macro, but instead of printing the output to the screen, it returns a `String` with  the contents
      // `format!` uses references, so it doesn't take ownership of any parameters unlike '+'
      
    /* Rust doesn't support direct indexing into Strings due to Strings being UTF-8 encoded 
    * which means that the size of chars in Strings aren't fixed
    */
       
    /* The Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:
    * [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    * If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    * ['न', 'म', 'स', '्', 'त', 'े']
    * There are six char values here, but the fourth and sixth are not letters:
    * They’re diacritics that don’t make sense on their own.
    * Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:
    * ["न", "म", "स्", "ते"]
    */
    
    /* A final reason Rust doesn’t allow us to index into a String to get a character
    * is that indexing operations are expected to always take constant time (O(1)).
    * But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through
    * the contents from the beginning to the index to determine how many valid characters there were.
    */
    
    /* Slicing Strings */
    let hello = "Здравствуйте";
    
    let s = &hello[0..4]; // s is a &str that contains the first 4 bytes of the string
    // each of those chars are 2 bytes, so s is "Зд"
    // so doing something like this:
    // let s = &hello[0..1];
    // would cause the program to panic as 'З' is 2 bytes not 1
    
    /* Iterating over Strings */
    for c in "Зд".chars(){
        println!("{c}");
    }
    
    for b in "Зд".bytes() {
        println!("{b}");
    }
    
    
    /* Hash Maps */
    use std::collections::HashMap;
    
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // Accessing values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("\n{team_name} score: {score}\n");
    
    // Iterating over key-value pairs in a hash map:
    for(key, value) in &scores {
        println!("{key}: {value}");
    } // iterating over a hash map happens in an arbitrary order
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // ownership of field_name and field_value are transferred to map because they are Strings (owned values that do not implement the `Copy` trait)
    
    /* Overwriting a value */
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{scores:?}");
    
    /* Adding a Key and Value Only if a Key isn't Present */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry() checks if the key exists, or_insert() inserts the value if it doesn't
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
    
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
    
    /* HashMap uses a hashing function called "SipHash" that can provide resistance to denial-of-service (DoS) attacks involving hash tables.
     * You can switch to another function by specifying a different hasher.
     * A hasher is a type that implements the `BuildHasher` trait.
     */
}
