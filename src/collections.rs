// Most other data types represent one specific value,
// but collections can contain multiple values.
// The data these collections point to is stored on the heap.

use std::collections::HashMap;

pub fn collections_main() {
    vectors_main();
    hashmaps_main();
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
    // because if no key found named "Jasmin" here then it should return None.
    match user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the DB"),
    }
}
