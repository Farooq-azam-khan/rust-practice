fn main() {
    let x = 5; 
    let y = {
        let z = 3; 
        z + 1
    }; 
    println!("{} {}", x, y); 
}
