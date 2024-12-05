/*compilation is slow coz of ownership checks

dangling pointer- pointer to a invalid data
double free error- deallocating memory which is occupied by some other program

every var in heap will have an owner in stack
if the owner of a var changes then the prev owner dies out

rust prevents having 2 owners for a var coz it causes issues like double free errors and dangling pointers
*/

fn main() {
    // let s1:String = String::from("hi");
    // let s2 = s1.clone(); //.clone() creates a copy
    // println!("Pointer: {:p}", s1.as_ptr());
    // println!("Pointer: {:p}", s2.as_ptr());
    let my_string:String = String::from("abhi");
    println!("Poiner: {:p}", my_string.as_ptr());
    takes_ownership(my_string.clone()); //clone() copies the var and is expensive
    println!("{}", my_string);
}

fn  takes_ownership(some_string: String) {
    println!("{}", some_string);
}