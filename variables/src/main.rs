// By default variables are immutable in rust
// We can make a variable mutable by using 'mut' keyword before it
// const is different from let as it is entriely immutable, even with mut keyword
// We also cannot any result of the function to a const variable but to let
// const variables are to be type annotated


fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{}", x);

    // const mut AGE: u32 = 25; // This will not work
    const AGE: u32 = 25;

    println!("{}", AGE);
}
