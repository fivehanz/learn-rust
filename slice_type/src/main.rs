fn main() {
    let s = String::from("What's up Hanz?");
    let word = first_word(&s[..]);
    println!("{} is {}", s, word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}