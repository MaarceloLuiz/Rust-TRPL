fn main() {
    let x = 5;
    println!("X value is {x}");
    let x = x+1;
    println!("X value is {x}");
    let x = x*2;
    println!("X value is {x}");

    let spaces= "    ";
    let spaces = spaces.len();
    println!("spaces length is {spaces}");

    let c = 'a'; //single quote instead of double quote
    println!("c value is {}", c);


    //tuple
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup value is {}, {}, {}", x,y,z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("values extracted by using a period `.` {}, {}, {}", five_hundred,six_point_four, one);

    //arrays
    let a = [1,2,3,4,5];
}
