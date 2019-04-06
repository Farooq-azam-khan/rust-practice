fn main() {
    let s1 = "abcde";
    let s2 = "abc"; 
    println!("{}", longest(s1, s2)); 
}

// <'a> is a generic lifetime 
// and &'a syas this ref has lifetime
fn longest <'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x }
    else { y } 
}
