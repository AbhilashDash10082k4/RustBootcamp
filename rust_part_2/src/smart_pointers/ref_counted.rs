/*-Rc<T> Reference Counted
-keeps count of owners of a data
-allows multiple "immutable" owners of a single data and also a single mutable ref chked during compile time
-takes a data and increases teh refernce count of the data and does not deep copies(clone) them
-this isnt thread safe, used only for 1 single thread
-for multi thread, Arc<T> is used */
use crate::smart_pointers::ref_counted::List::{Cons, Nil};
use std::rc::Rc; 
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
fn ref_counted() {
    let a = Rc::new(Cons(3, Rc::new(Cons(5, Rc::new(Cons(7, Rc::new(Nil)))))));
    {
        let _y = Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::clone(&a))))));
        print!("{}", Rc::strong_count(&a)); //prints the no.of owners/refernces of same data
    }
    let _z = Cons(3, Rc::new(Cons(4, Rc::new(Cons(5, Rc::clone(&a))))));

    print!("{}", Rc::strong_count(&a)); //prints the no.of owners/refernces of same data
}
