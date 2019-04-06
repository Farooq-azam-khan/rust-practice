enum Result<T, E> {
    OK(T), 
    ERR(E)
}

// for enums rust must know types in comiple time

fn main() {
    let mut r1: Result<i32, String> = Result::OK(5); 
    let r2: Result<i32, &str> = Result::ERR("bad"); 

    r1 = Result::ERR(String::from("test"))
}
