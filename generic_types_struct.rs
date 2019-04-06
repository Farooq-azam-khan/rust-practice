struct Point<T> {
    x: T, 
    y: T
}

impl<T> Point<T> {
    // no & because moving owership 
    fn swap_coords(self) -> Point<T> {
        Point { x: self.y, y:self.x }
    }
}

impl Point<f64> {
    fn vec_len(&self) -> f64 {
        ((self.x*self.x) + (self.y*self.y)).sqrt()
    }
}

fn main() {
    let p1 = Point { x:1, y:2 }; 
    let p2 = Point { x:1.0, y:3.13 }; 
    println!("p1: {} {} p2: {} {}", p1.x, p1.y, p2.x, p2.y); 
    
    let p1 = p1.swap_coords(); 
    println!("<{}, {}>", p1.x, p1.y); 

    let val = p2.vec_len(); 
    println!("{}",val);  
}
