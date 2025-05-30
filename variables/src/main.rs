// By default variables are immutable in rust
// We can make a variable mutable by using 'mut' keyword before it
// const is different from let as it is entriely immutable, even with mut keyword
// We also cannot any result of the function to a const variable but to let
// const variables are to be type annotated

// Shadowing varibles
// Shadowing allows to create a new variable using the name of a existing variable
fn main() {
    let mut x = 5;
    println!("{}", x);
    x = 10;
    println!("{}", x);

    // const mut AGE: u32 = 25; // This will not work
    const AGE: u32 = 25;

    println!("{}", AGE);

    // Shadowing
    let class_count = 30;
    println!("Total classes in a school {}", class_count);
    
    // class_count has shadowed the before class count variable
    // this also preserves the immutability of variables
    let class_count: &str = "thirty";
    println!("Total classes in a school {}", class_count);

}
