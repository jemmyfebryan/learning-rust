fn main() {
    let mut num: u32 = 0;
    let mut word: String = String::from("word");

    increase(&mut num);
    change_word(&mut word);

    println!("{}", num);
    println!("{}", word)
}

fn increase(num: &mut u32) {
    *num += 1;
}

fn change_word(word: &mut String) {
    *word = String::from("new word");
}