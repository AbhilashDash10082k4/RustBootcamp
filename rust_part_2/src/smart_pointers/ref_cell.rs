use std::{cell::{Ref, RefCell}, rc::Rc};
use crate::smart_pointers::ref_cell::List::{Cons, Nil};
/*-RefCell<T> -interior mutability
    -controlled inner mutability
    -can mutate an immutable data
    -tracks no. of immutable(can be many) and mutable(only 1) borrows
    -single threaded, RunTime panic
    -test doubles are used - (mock tests, other types that are used while testing, these tracks/records the events during a test to see if corect actions have taken place)
    -borrow rules are checked during runtime with contrast to during compile time for Box, Rc and all other use cases
    -adv of chking borrow rules in compile time- detecting errors early in code before execution
    -adv of chking borrow rules in run time- allowing certain memory safe opns
    -RefCell is used when the borrowing rules are followed but compiler doesnt understand
    -
*/
/*-e.g- Rate limiting in Rust(send msgs based on how close the current value is to the max val)*/
trait Messenger {
    fn send(&self, msg: &str);
}
struct Tracker<'a, T> {
    messenger: &'a T,
    value: f64,
    max: f64,
}
impl<'a, T> Messenger for Tracker<'a, T>
{
    fn send(&self, msg: &str) {
        /*custom methods to send msgs*/
        println!("{msg}");
    }
}
impl<'a, T> Tracker<'a, T>
{
    fn new(max: f64, messenger: &'a T) -> Self {
        Tracker {
            messenger,
            value: 0 as f64,
            max
        }
    }
    fn send_message(&mut self, value: f64) {
        self.value = value;
        let percentage = value as f64 / self.max as f64;
        if percentage > 0.50 {
            self.send("Used more than 50%");
        } else if percentage > 0.90 {
            self.send("Used more than 90%");
        } else if percentage > 1.0 {
            self.send("Quota ended!");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        msgs: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                msgs: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            /* let mut new_mock_messenger = MockMessenger::new(); -this cannot be used as this creates an empty instance of MockMessenger due to which the struct is not being implemented directly but an instance of it is being used

            self.msgs.push(String::from(msg)) -this line demands changing the &self to &mut self which would change the entire signature of send fn of Messenger which would change the meaning of entire code. So to access this mutably, RefCell is used

            borrow_mut will panic if any other borrow exists(2 mutable refs in same scope are not allowed)

            RefCell keeps the track/count of Ref<T> and RefMut<T> and when either immutable or mutable ref goes out of scope the count  = count-1. This impls Deref trait which allows to use as normal reference
            
            -runtime cost increases as it keeps tracking the no. of RefMuts and Refs, also borrowing errors are discovered later in production
            */
            self.msgs.borrow_mut().push(String::from(msg))
        }
    }
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = Tracker::new(100 as f64, &mock_messenger);

        limit_tracker.send_message(80 as f64);
        /*RefMut returned by borrow_mut and Ref returned by borrow- these are smart pointers(implements Deref trait for which they can be used as references) which keep track of no of mutable and immutable refs of inner items respectively */
        assert_eq!(mock_messenger.msgs.borrow().len(), 1);
    }
}

/*multiple mutable ownerships*/
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}
fn multiple_mutale_owners() {
    let val = Rc::new(RefCell::new(7));
    /*a needs to be owned multiple times so initialised with Rc::new*/
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    /*borrow_mut => provides the pointer RefMut which points to RefCell::new from Rc::new
    * => dereferencing operator -> points to inner val which is mutable coz of RefCell*/
    *val.borrow_mut() += 7;
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}