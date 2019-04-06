fn main() {
    let msg = String::from("Hello, World"); 

    let slc = get_slice(&msg, 0, 5); 

    println!("{}", slc); 
}


fn get_slice(w: &String, s: usize, e: usize) -> &str {
    &w[s..e]
}
