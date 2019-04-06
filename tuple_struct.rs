mod color; 

fn main() {
    let red = color::Color(255, 0, 0); 
    println!("Color: {} {} {}", red.0, red.1, red.2); 
}
