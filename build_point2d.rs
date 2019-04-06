mod point2d; 

fn main() {
    let pt = point2d::Pt2d(0.8, 1f64); 
    println!("point: <{}, {}>", pt.0, pt.1); 
}
