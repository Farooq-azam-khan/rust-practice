fn main() {
    let nums = [1,2,3,4,5,6,7,8,9]; 

    let tail = &nums[4..8]; 

    for n in tail.iter() {
        print!("{} ", n); 
    }
    println!(); 
}
