fn main() {
    println!("Hello, world!");

    another_function();
    third_function(5);
    fourth_function(6, 7);
}
//Function to print "Another function."
fn another_function(){
    println!("Another function.");
}
//Function to print out a 32 bit int
fn third_function(x: i32){
    println!("The value of x is: {}",x);
}
//Function to print out a 32 bit int
fn fourth_function(x: i32, y: i32){
    println!("The value of x is: {}",x);
    println!("The value of x is: {}",y);
}
