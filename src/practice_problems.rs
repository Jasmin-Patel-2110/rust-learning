use std::collections::HashMap;

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

    let res_hm = tups_to_hashmap(vec![
        (String::from("apple"), 1),
        (String::from("banana"), 2),
        (String::from("apple"), 3),
        (String::from("orange"), 4),
        (String::from("banana"), 5),
    ]);
    println!("Created HashMap: {:?}", res_hm);
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

// Write a function that takes a vector of tuples (each tuple containing a key and a value)
// and returns a Hashmap where the keys are the unique keys from the input tuples and
// the values are vectors of all corresponding values assosiated with each key.
fn tups_to_hashmap(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut hm = HashMap::<String, Vec<i32>>::new();

    for (key, val) in pairs {
        let res = hm.get_mut(&key);

        match res {
            Some(vec) => {
                vec.push(val);
            }
            None => {
                hm.insert(key, vec![val]);
            }
        }
    }
    return hm;
}
