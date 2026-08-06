fn main() {
    loop_main();
    ownership_main();
    borrow_main();
    struct_main();
    enum_and_patt_matching_main();
}

// Loops and Conditionals
fn loop_main() {
    let sentence: String = String::from("My name is Jasmin.");
    let first_word: String = get_first_word(sentence);

    let n = 100;
    for i in 0..n {
        println!("number: {i}");
    }

    println!("First word is {first_word}");
}

fn get_first_word(sentence: String) -> String {
    let mut ans: String = String::new();

    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;
}

// Ownership
fn ownership_main() {
    normal_assignment();
    passing_to_fn();
}
// 1. normal assignment
fn normal_assignment() {
    let s1 = String::from("My String");
    println!("s1 before assignment: {s1}");
    let s2 = s1;
    // println!("s1 after assignment: {s1}"); // error[E0382]: borrow of moved value: `s1`
    //                                 ^^ value borrowed here after move
    println!("s2 after assignment: {s2}");
}
// 2. passing to function
fn passing_to_fn() {
    let my_string: String = String::from("Hello");
    takes_ownership(my_string); // Ownership passed here
    // takes_ownership(my_string.clone()); // Creates and passes a copy instead of ownership, but use only when needed.
    // println!("string after passing to fn: {my_string}"); // error[E0382]: borrow of moved value: `my_string`
    //                                        ^^^^^^^^^ value borrowed here after move

    // if want ownership back -> make my_string `mut` and then
    // my_string = takes_ownership(my_string);
    // println!("string after passing to fn: {my_string}");
}

fn takes_ownership(string: String) -> String {
    //             ^^^^^^ this parameter takes ownership of the value
    println!("String passed in Funtion: {string}");
    // return string; // return the parameter if want to pass the ownership back or use borrow instead.
    string // another way of returning by removing `return` keyword and `;`.
}

// Borrowing and References
fn borrow_main() {
    normal_reference();
    mut_reference_01();
    mut_reference_02();
}

// simple reference
fn normal_reference() {
    let s1: String = String::from("My String");
    let s2: &String = &s1;
    // s2 (borrower) -> s1 (owner) -> "My String" (Heap)
    println!("s1 after borrowed by s2: {s1}");
    println!("s2 after borrow: {s2}");
}

// mutable reference
// If one mutable reference is there then it can't have any mutable or immutable reference.
fn mut_reference_01() {
    let mut s1: String = String::from("Hello, ");
    println!("s1 before update_str: {s1}");
    let s2: &mut String = &mut s1;
    // let s3: &String = &s1; // cannot borrow s1 as immutable because it is also borrowed as mutable
    // update_str(&mut s1); // cannot borrow `s1` as mutable more than once at a time
    s2.push_str("Something");
    println!("s2: {s2}");
    println!("s1 after update_str: {s1}");
}

fn mut_reference_02() {
    let mut s1: String = String::from("Hello, ");
    println!("s1 before update_str: {s1}");
    // if immutable borrow here then `update_str` and `s2` can't borrow because it is used after both declared.
    // let s3: &String = &s1;
    update_str(&mut s1); // this fn's mutable borrow ends here...
    let s2: &mut String = &mut s1; // so mutable borrow possible for s2.
    s2.push_str("Something");
    // println!("s3: {s3}");
    println!("s1 after update_str: {s1}");
}

fn update_str(s: &mut String) {
    s.push_str("World!");
}

// Struct
// similer to classes in JS/TS.
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }
    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

struct unit_struct; // struct with just implementations (functions only, no attributes)

impl unit_struct {
    fn f1(&self) {}
    fn f2(&self) {}
    fn f3(&self) {}
}

fn struct_main() {
    let r1 = Rect {
        width: 30,
        height: 20,
    };

    println!("area of r1 is {}", r1.area());
    println!("perimeter of r1 is {}", r1.perimeter());

    let u = unit_struct {}; // no attributes/variables
    u.f1();
    u.f2();
    u.f3();
}

// enums and pattern matching in enums
enum Direction {
    up,
    down,
    left,
    right,
    none, // stationary
}

fn move_around(direction: Direction) {
    // logic to move the character around
}

// enums with values
enum Shape {
    Circle(f64),         // associated data (radius)
    Square(f64),         // associated data (length)
    Rectangle(f64, f64), // associated data (width, height)
}

// function to calc area based on shape
fn calc_area(shape: Shape) -> f64 {
    // pattern matching in enum
    let ans = match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(length) => length * length,
        Shape::Rectangle(width, height) => width * height,
    };
    return ans;
}

fn enum_and_patt_matching_main() {
    let my_direction = Direction::none;
    move_around(my_direction);

    let circle = Shape::Circle(5.4);
    let square = Shape::Square(8.0);
    let rect = Shape::Rectangle(3.5, 6.4);

    println!("area of circle: {}", calc_area(circle));
    println!("area of square: {}", calc_area(square));
    println!("area of rect: {}", calc_area(rect));
}
