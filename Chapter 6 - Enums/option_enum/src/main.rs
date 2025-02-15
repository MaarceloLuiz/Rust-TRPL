fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    
    let absent_number: Option<i32> = None;

    println!("{:?}", some_number);
    println!("{:?}", some_char);
    println!("{:?}", absent_number);
}

//The enum option is already defined by the standard library as follows:

/*
enum Option<T>{
    None,
    Some(T),
}
 */