fn main() {
    println!("Hello, world!");

    another_function(5);

    println!("the value of plus_one() is {}", plus_one(5))
}
fn another_function(x: i32){
    println!("another function! The value of x is {x}");

    let y = {
        let z = 3;
        z+1
    };

    println!("testing expression y value {}", y);
}
fn plus_one(x: i32) -> i32{
    //return x+1;
    x+1
}
