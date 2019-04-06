fn main() {
    let arr = [1,2,3,4,5]; 
    println!("{}", max_val_i32(&arr)); 

}

fn max_val_g<T> (arr: &[T]) -> T {
    let mut largest = arr[0]; 
    for &n in arr.iter() {
        if n > largest { largest = n; } 
    }
    largest
}

fn max_val_i32 (arr: &[i32]) -> i32 {
    let mut largest = arr[0]; 
    for &n in arr.iter() {
        if n > largest { largest = n; }
    }
    largest 
}
