struct Person {
    name: &str, 
    age: u8
}

fn main() {
    let user1 = Person {
        name: "Farooq", 
        age: 19
    };
    println!("{} is {}", user1.name, user1.age); 

}
