// By default variables are immutable in rust


// We can make a variable mutable by using 'mut' keyword before it


fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{}", x);
}
