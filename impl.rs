struct Rect {
    width: f64, 
    height: f64
}

impl Rect {
    fn area (&self) -> f64 {
        self.width * self.height
    }
}


fn main() {
    let r1 = Rect { width: 5.0, height: 3.0 };
    println!("Area = {}", r1.area()); 
}
