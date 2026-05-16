pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where 
    T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: RefCell::new(vec![]), }
        }
    }

    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. 
    // Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active.
    // When a Ref<T> value goes out of scope, the count of immutable borrows goes down by 1.
    // Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) { // immutable borrow of self
            self.sent_messages.borrow_mut().push(String::from(msg)); // here we're mutating self by using `.borrow_mut()` to get a mutable reference to the value inside RefCell<Vec<String>>
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut(); // This would panic! because there are 2 mutable references which goes gainst the borrowing rules
            // The code panicked with the message "already borrowed: BorrowMutError".
            // This is how RefCell<T> handles violations of the borrowing rules at runtime.
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // `.borrow()` gives us an immutable reference to the vector
    }
}