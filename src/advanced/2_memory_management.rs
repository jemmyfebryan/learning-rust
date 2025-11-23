// // Ownership Model
// fn main() {
//     let mut word: String = String::from("word");

//     word = print(word);

//     println!("{}", word);
// }

// fn print(word: String) -> String {
//     println!("{}", word);
//     word
// }


// Reference Model
fn main() {
    let word: String = String::from("word");

    print(&word);

    println!("{}", word);
}

fn print(word: &String) {
    println!("{}", word);
}