struct Person {
    name: String, 
    age: u8
}

fn main() {

    let user1 = Person {
        age: 19, 
        name: String::from("farooq")
    };

    println!("user1 is {} and {} years old", user1.name, user1.age); 

}
