fn main() {

    // Rules of ownership
        // Each value in rust has a variable that is knows as its owner
        // There can only be one owner at a time
        // When the owner goes out of scope the value is dropped  

        // {
        //     let str = "Hello";
        //     println!("{}", str);
        // }
        // println!("{}", str);


        let s1 = String::from("hello");
        let s2 = s1; // Move (not shallow copy, meaning s1/s1's value has been MOVED to s2)

        println!("{}, world!", s1);
    }