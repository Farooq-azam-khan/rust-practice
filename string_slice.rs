fn main() {
    let msg = String::from("Hello, World"); 
    let hello = &msg[0..5]; 
    let world = &msg[7..]; 


    println!("{}", hello); 
    println!("{}", world);
}
