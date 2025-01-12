//1- USING TUPLES:

/* fn main() {
    let rectangle = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rectangle));
}

fn area(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}
 */

//2- USING STRUCTS:
/*  #[derive(Debug)]
 struct Rectangle{
    width: u32,
    height: u32,
}

 fn main() {
    let rectangle = Rectangle{
        width: 30,
        height:50,
    };
    println!("{:?}", rectangle);
    println!("{:#?}", rectangle);

    println!("The area of the rectangle is {} square pixels.", area(&rectangle));
}

fn area(rectangle:&Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
} */

//3- USING STRUCTS AND METHODS:
#[derive(Debug)]
struct Rectangle{
   width: u32,
   height: u32,
}

impl Rectangle{
    fn new(width: u32, height: u32) -> Self { //similar to a Constructor
        return Self{width, height};
    }

    fn area(&self) -> u32 {
        return self.width * self.height;
     }
}

fn main() {
   let rectangle = Rectangle::new(30, 50);
   println!("{:?}", rectangle);
   println!("{:#?}", rectangle);

   println!("The area of the rectangle is {} square pixels.", rectangle.area());
}