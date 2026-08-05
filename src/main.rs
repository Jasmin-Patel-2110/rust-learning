fn main() {
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
