fn main() {
    // let result = loop_up();
    // println!("Final number is {}", result);

    for_loop();
}

fn my_function(num1: i32, num2: i32) -> i32 {
    num1 + num2 
}


fn loop_up() -> i32 {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;
        println!("{}", counter);
        if(counter == 10) {
            break counter;
        }
    };

    result
}


fn for_loop() {
    let a = [100, 200, 300, 400, 500];

    for digit in a.iter() {
        println!("{}", digit);
    }
}