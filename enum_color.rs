enum Color {
    T(u8, u8, u8), 
    S(String)
}


fn main() {
    let num = Color::T(255, 0, 0); 
    let str = Color::S(String::from("255, 0, 0")); 
    print_color(num); 
    print_color(str); 
}

fn print_color(c: Color) {
    match c {
        Color::T(a,b,c) => {
            println!("{} {} {}", a,b,c); 
        },
        Color::S(a) => {
            println!("{}", a); 
        }
    }
}
