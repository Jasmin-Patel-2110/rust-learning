pub fn practice_problems_main() {
    // Fibbonacci
    let num1: u32 = 3;
    println!("fibbonacci num of {}: {}", num1, fib(num1));

    // string length
    let s1: String = String::from("Hello");
    println!("Length of the string '{}' is {}", s1, get_string_length(&s1));
}

// Write a function fib that finds the fibbonacci of the number is taks as input.
fn fib(num: u32) -> u64 {
    if num == 0 {
        return 0;
    }
    if num == 1 {
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

// fn get_string_length that takes a string as an input and returns its length.
fn get_string_length(s: &String) -> usize {
    // let mut length = 0;
    // for _ in s.chars() {
    //     length += 1;
    // }
    // return length;
    s.chars().count()
}
