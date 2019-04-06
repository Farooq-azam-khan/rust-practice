fn main() {
    let s1 = "abcdef"; 
    let s2 = "abc"; 

    println!("{}", longest(s1, s2)); 
}


fn longest(x: &str, y:&str) -> &str {
    if x.len() > y.len() { x }
    else { y } 
}
