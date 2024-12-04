/*memory management
programmes get executed in RAM
ways of memory management-
1.garbage collection- initially a variable is stored in RAM but after the execution of the fn it is removed from RAM(memory is deallocated) and is garbage collected, this is slow

2.manual- mem management in c ,leads to dangling pointers

3.rust way- has its own ownership for memory management makes it safe to memory errors

JARGONS-
   a.Mutability- changeability, immutability optimizes code and compiler faster
   
   b.Stack vs Heap- 
   Stack- more organized, mem allocated in a stack, the 1st var will be stored at the bottom of the stack, only var of fixed size are stored, int , bools, 
   fast allocation and deallocation
   each fn has a diff stack frame

   Heap- for var of varying length, strs and vectors are stored
   in string- 1st a stack frame is allocated and then it points towards a heap to store the str, in the stack the address of indiv char(POINTER), len and capacity are stored 
   
   runtime(cargo run)- code is actually running, compile time(cargo build)- code is getting compiled together
   */

fn main() {
    stack_fn();
    heap_fn();
    update_string_fn();
}

fn stack_fn() {
    //declaring integers
    let a = 3;
    let b = 4;
    let c = a + b;
    println!("{}", c);
}

fn heap_fn() {
    //combo of strings
    let a = String::from("abhilash");
    let b = String::from("dash");
    let full_name = format!("{} {}", a, b);
    println!("Heap fn: '{}'", full_name);
}

fn update_string_fn() {
    let mut str1:String = String::from("abhilash dash is ");
    println!("Capacity: {}, Length: {}, Pointer :{:p}", str1.capacity(), str1.len(), str1.as_ptr());

    str1.push_str("a senior rust developer");
    println!("{}", str1);
    
    println!("Capacity :{}, Length :{}, Pointer :{:p}", str1.capacity(),str1.len(), str1.as_ptr() );
}