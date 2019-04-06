fn main() {
    let mut s = String::from("H"); 
    s = string_pass(s); 

    println!("{}", s); 
}


fn string_pass(str: String) -> String {
    println!("string_pass: {}", str); 
    str// returns onwership to caller
}
