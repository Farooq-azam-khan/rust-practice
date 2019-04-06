fn main() {

    let mut s1 = String::from("Hello"); 
    let len = string_len(&mut s1); 

    println!("{} has {} chars", s1, len); 
}


fn string_len(word: &mut String) -> usize {
    word.push_str(", world"); 
    word.len()
}
