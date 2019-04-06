enum IpType {
    V4, 
    V6
}

struct IpAddr {
    kind: IpType, 
    address: String
}

fn main() {
    let ip1 = IpAddr {
        kind: IpType::V4, 
        address: String::from("127.0.0.1")
    };

}
