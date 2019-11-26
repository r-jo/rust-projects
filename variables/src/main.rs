fn main() {
    //the 'mut' keyword allows a variable to change value
    //otherwise the compiler will permanently bind that value
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //when declaring a const, you must ALWAYS define its type
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    //Example of shadowing
    //It's different from mut because it will become immutable after the operation
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The (shadow) value of x is: {}", x);

    //Another example
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The (shadow) value of spaces length is: {}", spaces);
}
