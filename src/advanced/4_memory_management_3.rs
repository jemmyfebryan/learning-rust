fn main() {
    // let x: u32 = 0;
    // let x: String = String::from("x");

    // println!("{}", x)


    let word: String = String::from("word");
    let word_str_1 = &word[..];
    let word_str_2 = word_str_1;

    let word_uppr = word.to_uppercase();

    println!("{}", word_str_1);
    println!("{}", word_str_2);
    println!("{}", word_uppr);
}