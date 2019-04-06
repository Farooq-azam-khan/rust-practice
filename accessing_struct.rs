mod person; 

fn main() {
    let mut user1 = person::Person {
        age: 19, 
        name: String::from("Farooq")
    }; 
    user1.name = String::from("azam"); 
    println!("user1 is {} and {} years old", user1.name, user1.age); 
}
