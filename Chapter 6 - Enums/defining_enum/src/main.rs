fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    println!("{:?}", home);
}

#[derive(Debug)]
enum IpAddrKind{
    V4(String),
    V6(String),
}