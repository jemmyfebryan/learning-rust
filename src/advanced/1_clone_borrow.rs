fn main () {
    let num_1: u32 = 0;

    print_1(num_1);

    println!("{}", num_1);

    let word_1: String = String::from("word");

    print_2(word_1);

    println!("{}", word_1);

    // println!("{}", num_1);
    // println!("{}", num_2);
    // println!("{}", word_1);
    // println!("{}", word_2);
}

fn print_1(num: u32) {
    println!("{}", num);
}

fn print_2(word: String) {
    println!("{}", word);
}