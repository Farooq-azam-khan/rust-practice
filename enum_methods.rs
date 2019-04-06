enum IpType {
    V4(u8, u8, u8, u8), 
    V6(String)
}
/*
impl IpType {
    fn route(&self) {
    }
}
*/

fn main() {
    let v4 = IpType::V4(127, 0, 0, 1); 
    let v6 = IpType::V6(String::from("127.0.0.1")); 

    print_ip(v4); 
    print_ip(v6); 
}


fn print_ip(ip: IpType) {
    match ip {
        IpType::V4(a,b,c,d) => {
            println!("{}:{}:{}:{}", a,b,c,d); 
        },
        IpType::V6(a) => {
            println!("{}", a); 
        }
/*
        _ => {
            println!("opps."); 
        }
*/
    }
}
