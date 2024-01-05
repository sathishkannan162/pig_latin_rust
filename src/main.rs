fn main() {
    let hello = "hello world from pigs apple for us";
    let mut pig_latin = String::new();

    for word in hello.split_whitespace() {
        let first_letter = word.chars().nth(0).expect("There is no word");
        match first_letter {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                let n = word.to_owned() + "-hay ";
                pig_latin.push_str(&n);
            }
            _ => {
                let m = &word[1..];
                let n = m.to_owned() + "-" + &String::from(first_letter) + "ay ";
                pig_latin.push_str(&n);
            }
        };
    }
    println!("{pig_latin}");
}
