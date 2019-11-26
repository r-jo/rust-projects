fn main() {
    //Floating points
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    //Numeric Operations
    // addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    //Boolean
    let _t = true;
    let _f: bool = false; //with explicit type annotation

    //Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    println!("The emoji is: {}", _heart_eyed_cat);

    //Compound Types
    //Tuple Type
    //let _tup: (i32, f64, u8) = (500, 6.4, 1);

    let _tup = (500, 6.4, 1);

    let (_x, _y, _z) = _tup;

    println!("The value of y is: {}", _y);

    //Arrays
    let _a = [1,2,3,4,5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    //specifies the type of element ';' then how many elements
    let _b: [i32; 5] = [1,2,3,4,5];

    //specifies the same element ';' then how many elements
    //Same as writing:
    //let _d = [3,3,3,3,3];
    let _d = [3; 5];
    //accessing elements of an array
    let _first = _b[0];
    let _second = _b[1];
}
