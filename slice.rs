fn main() {

    let nums = [1,2,3,4, 5, 6, 7]; 
    let subset = get_slice(&nums, 1, 5); 

    for n in subset.iter() {
        print!("{} ", n); 
    }
    println!(); 

}


fn get_slice(a: &[i32], s: usize, e: usize) -> &[i32] {
    &a[s..e]
}
