fn main() {
    let mut s1 = String::from("w"); 
    s1.push_str(" s1 "); 
    let r3 = &mut s1; 
    r3.push_str(" r3 ");

    println!("{}", r3); 
}
