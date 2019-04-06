fn main() {
    let s1 = String::from("w"); 

    println!("{} has {} chars", s1, string_len(&s1)); 
}

fn string_len(word: &String) -> usize {
        word.len() 
}
