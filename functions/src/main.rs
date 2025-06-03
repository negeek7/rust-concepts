fn main() {
    println!(my_function(20, 25));
}

fn my_function(num1: u32, num2: u32) -> u32 {
    let sum = num1 + num2;

    // return sum;

    sum // another way of returning in rust
}
