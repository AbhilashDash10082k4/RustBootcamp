/* Collections */
fn main() {
    //intializing vector using macros
    let numbers = vec![1,2,3];
    println!("{:?}", numbers);

    let mut vec = Vec::new();
    
    let vec_2 = &mut vec;
    
    for val in 0..13 {
        let vec_2 = vec_2.push(val);
    }
    println!("MUTABLE REFERENCE OF VEC: {:?}", vec_2);

    let even_vec_var = even_vec(vec_2);
    println!("NEW_VEC: {:?}", even_vec_var);
    println!("OG VEC: {:?}", vec);
}
//fn that takes a vec as ip and returns a vec with even nos
fn even_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val); /* ' * ' -dereferences the val */
        }
    }
    return new_vec;
}