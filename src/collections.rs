// Most other data types represent one specific value,
// but collections can contain multiple values.
// The data these collections point to is stored on the heap.

pub fn collections_main() {
    vectors_main();
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
