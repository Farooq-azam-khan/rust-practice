enum IpType {
    V4(u8, u8, u8, u8), 
    V6(String)
}

fn main() {
    let home = IpType::V4(127, 0, 0, 1); 
}
