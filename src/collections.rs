// Most other data types represent one specific value,
// but collections can contain multiple values.
// The data these collections point to is stored on the heap.

use std::collections::HashMap;

pub fn collections_main() {
    vectors_main();
    hashmaps_main();
    interators_main();
}

// vectors

// Works just like dynamic arrays.
// Store multiple values in single data structure
// that puts all the values next to each other in memory.

fn vectors_main() {
    let mut v: Vec<i8> = Vec::new();
    v.push(10);
    v.push(23);
    v.push(6);
    println!("v = {:?}", v);
    // for now understand that if "{}" doesn't work then try "{:?}"
}

// Hashmaps

// Key Value pairs.
// Similar to objects in JS, Dict in Python and HashMaps in Java

fn hashmaps_main() {
    let mut users = HashMap::new();

    users.insert("Jasmin", 21);
    users.insert("Doe", 20);
    users.insert("Jems", 23);

    // {
    //  "Jasmin" : 21,
    //  "Doe"    : 20,
    //  "Jems"   : 23
    // }

    let user_age = users.get("Jasmin");
    // .get(k) returns a Option, so pattern match it for output.
    // because if no key named "Jasmin" found here then it should return None.
    match user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the DB"),
    }
}

// Iterators

// iter() method provides a way to iterate over the elements of a collection
// by borrowing them.
//
//
// Iterators are lazy, meaning if you just defined an iterator like below,
// you have done nothing.
//
// let v1 = vec![1, 2, 3];
// let v1_iter = v1.iter(); // nothing is done in this step.
//
// for value in v1_iter {
//      println!("{}", value);
// }
//
// Think of an iterator as another type just like Vector, HashMap, i32, etc.
// v1_iter has no effect untill you call methods that consume the
// iterator to use it up.
//

fn interators_main() {
    // you can't mutate the variables since we have
    // an immutable reference to the internal elements.
    let mut v1 = vec![1, 2, 3];
    // If you want mutable reference then use `.iter_mut()` method.
    println!("v1 before referenced by iterator {:?}", v1);

    let v1_iter = v1.iter_mut();

    for val in v1_iter {
        *val = *val + 1;
    }
    println!("v1 after referenced by iterator {:?}", v1);

    // Iterating using .next() method
    let nums = vec![1, 2, 3];
    let mut iter = nums.iter();

    while let Some(val) = iter.next() {
        // for loop hides this complexity
        print!("{}", val);
    }
}
