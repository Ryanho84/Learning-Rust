fn main() {
    //println!("Hello, world!");
    let x = 5;
    println!("x is {}", x);

    //x = 6 ; error : can not assign twice to immutable variable
    //println!("x is {}", x);

    let mut y = 6;
    println!("y is {}", y);

    y = 7; // correct, mut keyword
    println!("y is {}", y);

    const MAX_POINTS : u32 = 100_000;

    println!("const num is {}", MAX_POINTS);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("len is {}", spaces);

    let guess : u32 = "42".parse().expect("Not a number");  //correct
    //let guess = "42".parse().expect("Not a num");    //error: parse need type

    println!("guess is {}", guess);

    let cat = "😻";
    let length = cat.len();
    println!("cat is {}",cat);
    println!("cat_len is {}", length);
}
