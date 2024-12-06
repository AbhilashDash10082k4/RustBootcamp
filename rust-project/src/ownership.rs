/*compilation is slow coz of ownership checks

dangling pointer- pointer to an invalid data, a var in heap has 2 owners, one of them cleans it while getting deallocated and the other still points towards the var in heap, this is called as dangling pointers

double free error- deallocating memory which is occupied by some other program, at first the memory allocated to the code gets free and some other program occupies the heap but the pointer of the code points towards the program and tries to clean it 

every var in heap will have an owner in stack
if the owner of a var changes then the prev owner dies out

rust prevents having 2 owners for a var coz it causes issues like double free errors and dangling pointers
*/

fn main() {
    let s1:String = String::from("hi");
    println!("Pointer: {:p}", s1.as_ptr());

    let s2 = s1; //.clone() creates a copy
    println!("Pointer: {:p}", s2.as_ptr());

    let mut my_string:String = String::from("abhi");//owner of the str-> my_string,
    //mut makes the var flexible to take back the ownership, accept his gf back even after she cheated on him
    println!("Poiner: {:p}", my_string.as_ptr());

    //giving ownership back to my_string
    my_string = takes_ownership(my_string); //now ownership transferred to some_string;
    println!("{}", my_string);
}

fn  takes_ownership(some_string: String)-> String {
    println!("{}", some_string);
    //giving ownership back to my_string
    return some_string;
}