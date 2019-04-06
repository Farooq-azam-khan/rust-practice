fn main() {
    let s1 = String::from("W"); 
    let (len, s2) = str_len(s1); 

    println!("{} is {} long", s2, len); 
}


fn str_len(str: String) -> (usize, String) {
    (str.len(), str)     
}
