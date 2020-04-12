#![allow(unused_variables)]

fn main() {
    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    let s = String::from("hello world");

    let first_word_idx = first_word(&s);

    // s.clear(); // after clear the first word idx is not valid in string s

    println!("first word is {}", &s[..first_word_idx])
}
