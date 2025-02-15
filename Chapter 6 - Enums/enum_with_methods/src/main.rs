fn main() {
    let message = Message::Write(String::from("Hello There!"));
    println!("{:?}", message);
    
    message.call()
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn call(&self){
        println!("Testing call function");
    }
}
