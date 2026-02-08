use crate::smart_pointers::boxx::List::{Cons, Nil};
use std::ops::Deref;
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

/*Smart pointers- Used to point towards a data type
- Owns the data they are pointing to
- Allows multiple owners of a data, keep track of no. of owners the data
- implemented using structs, implements Deref(to use the owned data type as a ref ) and Drop(code to run after the smart pointer goes out of the scope) traits
Rc, RefCell, Cell, Box<T>
*/
/*-Box<T>
    -immutable and mutable ref in compile time
    -store data in heap,
    -used to use a data type whose size is not known at compile time
    -transfer ownership of large amount of data and make sure that the data is not copied (dat ais stored in heap and a small amount of pointer data stored on stack is copied)
-Box is stored on stored on stack and the data is stored on heap
    -Box<7> -> 7 is stored on heap and Box on stack
    -used with recursive cons list-
    -storing vals of non recursive data types -size of an enum is defined by the largest size used by any of its variant
    -for recursive types, the size determination process goes on until an end is reached, which is infinite.
    -Box<List> -stores pointer to the List ,which is stored on heap. It si like storing a val one beside another rather than storing it one inside other. Box is a pointer and stored on stack whose size is known at compile time
    -Box -> implements Deref(allowed to be used as a reference) and Drop(data on heap is dropped after Box goes out of scope)
-treating smart pointers as references -
    -deref coercion -converting a type that implements Deref trait to convert into another type that a fn can accept as an argument. A sequence of call is made to deref method to convert the data type into another data type -can be called many times to match the param type of the specified fn. This occurs at compile time so no runtime overhead is caused
    -DerefMut trait for mut references
        -&T to &U where T: DerefMut<Target = U>
        -&mut T to &mut U where T: DerefMut<Target = U>
        -&mut T to &U where T: DerefMut<Target = U> -> converts a mutable type to immutable type but not otherwise
-Drop trait-
    -happens once, in reverse order(stack style cleanup), consumes the data in the real implementation which takes it out of scope
    -called automatically when the code goes out of scope
    -impl<T> Drop for CustomBox<T> {
        fn drop(x: &mut self) {
            print!("Dropping {}", self.0);
        }
    }
    -std::mem(c) -> forced drop call to clear memory early in code
*/
fn boxx() {
    //cannot define as size is not known at compile time, requires - enum List {Cons(i32, Box<List>), Nil};
    let _x = Cons(1, Box::new(Cons(2, Box::new(Cons(7, Box::new(Nil))))));
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(x, *y); //-> * is used to track down the val that y is referencing to in order to compare with original val of x
    
    /*-custom pointer and deref trait impplementation -*/
    struct CustomBox<T>(T); //tuple struct
    impl<T> CustomBox<T> {
        fn new(x: T) -> CustomBox<T> {
            CustomBox(x)
        }
    }
    impl<T> Deref for CustomBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
            //.0 accesses the 1st val of teh tuple struct
            //returns a reference to the inner data type rather than returning the entire data which would cause the ownership to change
        }
    }
    fn hello(x: &str) {
        println!("{x}");
    }
    let x = CustomBox::new(String::from("AD"));
    hello(&x); //deref called multiple times to convert CustomBox<String> -> String -> &str
}
