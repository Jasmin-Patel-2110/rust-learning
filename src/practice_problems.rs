pub fn practice_problems_main() {
    // Fibbonacci
    let num1: u32 = 3;
    println!("fibbonacci num of {}: {}", num1, fib(num1));

    // string length
    let s1: String = String::from("Hello");
    println!("Length of the string '{}' is {}", s1, get_string_length(&s1));

    // take vec of nums and return vec of only even nums
    let res_vec = to_even_vec(&vec![12, 23, 43, 44, 65, 66, 77, 82]);
    println!("result vec with only even nums: {:?}", res_vec);
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

// Write a function that takes a vector as input and returns a vector with even values.
fn to_even_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
            // here * used for dereferencing the borrow
            // val is a pointer to the number
        }
    }

    return new_vec;
}
