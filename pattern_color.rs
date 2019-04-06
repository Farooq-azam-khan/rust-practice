fn main() {
    println!("{}", get_color((255, 0,0))); 
    println!("{}", check_red((255, 0,0))); 
}


fn get_color(vals: (u8, u8, u8)) -> String {
    match vals {
        (255, _, _) => String::from("Red"), 
        (_, 255, _) => String::from("Green"), 
        (_, _, 255) => String::from("Blue"), 
        (_, _, _) => String::from("Not Primary")
    }
}

fn check_red(vals: (u8, u8, u8)) -> String {
    match vals {
        (a, _, _) => {
            if a > 200 { String::from("bright red") }
            else if a > 100 { String :: from("medium red") }
            else { String::from("Dark red") }
        }
    }
}
