/* types of iterators-
    iterating using for loops
    creating an iterator- var.iter()
*/
fn main() {
    let mut vec_1 = Vec::new();
    for val in 0..13 {
        vec_1.push(val);
    }
    println!("{:?}", vec_1);

    let vec_1_iterator = vec_1.iter();
    // println!("{:?}", vec_1); //vec_1_iterator is a reference of vec_1

    for val in vec_1_iterator {
        if val % 2==0 {
            print!("{} ", val);
        } 
    }

    //mutable iterator
    let mut vec_1_mut_iterator = vec_1.iter_mut(); //mutable iterator

    // for val in vec_1_mut_iterator {
    //     *val = *val + 1;
    // }

    // println!("\n{:?}", vec_1);

    while let Some(val) = vec_1_mut_iterator.next() {
        print!("{} ", val);
    }

    //into.iter()
    let num_into_iter_vec = vec![10, 20, 30];
    let num_into_iter = num_into_iter_vec.into_iter(); //ths iterator takes the ownership of num_into_iter_vec

    for val in num_into_iter {
        print!("{} ", val*2);
    }

    //sum-> consuming adapter
    let num_2_sum = vec![1,2,3,4,5,6,7];
    let num_2_sum_iter = num_2_sum.iter();
    let sum: i32 = num_2_sum_iter.sum();
    println!("\n{}", sum);

    //iterator adaptor- returns iterator for an iterator
    let v1_iterator_adaptor = vec![1,2,3,4,5,6,7];
    let v1_iterator = v1_iterator_adaptor.iter();

    let v1_iterator2 = v1_iterator.map(|X| X+1);

    println!("{:?}", v1_iterator2);
    for i in v1_iterator2 {
        print!("{} ", i);
    }

    //filter
    let v1_iterator_filter = vec![1,2,3,4,5,6,7];
    let v1_iter_filter = v1_iterator_filter.iter();
    let v1_iter = v1_iter_filter.filter(|X| *X % 2 ==0);
    
    for val in v1_iter {
        print!("\n{} ", val);
    }

    //filter all odd values then double each and create a new vec
    let mut vec_assign = Vec::new();
    for i in 0..15 {
        vec_assign.push(i);
    }
    println!("{:?}", vec_assign);

    let vec_assign_iter = vec_assign.iter();
    let vec_assign_iter_adaptor = vec_assign_iter.filter(|X| *X % 2 != 0);

    let mut odd_vec = Vec::new();
    for val in vec_assign_iter_adaptor {
        odd_vec.push(val*2);
    }
    println!("{:?}", odd_vec);
    
}