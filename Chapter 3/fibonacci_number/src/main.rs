use std::io;

fn main() {
    loop{
        println!("Type a number");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("error to read line");

        let number:u32 = match number.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib(number));
        break;
    }
}

fn fib(n:u32) -> u32{
    if n == 0{
        return 0;
    }

    if n == 1{
        return 1;
    }

    return fib(n-1) + fib(n-2);
}
