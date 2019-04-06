mod person; 

fn main(){
    let user1 = build_person(String::from("farooq"), 19); 
    println!("Name: {}, age: {}", user1.name, user1.age); 

    let user2 = build_person_shortcut(String::from("azam"), 19); 
    println!("Name: {}, age: {}", user2.name, user2.age); 
}


fn build_person(name: String, age: u8) -> person::Person {
    person::Person { age: age, name: name }
}


fn build_person_shortcut(name: String, age: u8) -> person::Person {
    person::Person {age, name}
}
