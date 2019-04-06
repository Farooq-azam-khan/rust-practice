fn main() {
    let r: &i32; 
    {
        let x = 5; 
        r = &x; 
    }
    println!("r: {}", r); 
}


