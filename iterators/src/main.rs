

fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }

    // pub trait Iterator {
    //     type Item;

    //     fn next(&mut self) -> Option<Self::Item>;
    // }

    // Calling the map method to create a new iterator, and then calling the collect method to consume the new iterator and create a vector
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // Iterators are lazy, they don't do anything until something consumes it like collect(), sum(), max(), find(), etc. (consuming adapters)
    // Things like map(), filter(), take() are lazy adaptors, they just add to the description without running anything
    // Because all iterators are lazy, you have to call one of the consuming adapter methods to get results from calls to iterator adapters
    
    for v in &v2 {
        println!("{v}");
    }

    
    
}
