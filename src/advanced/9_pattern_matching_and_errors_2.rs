fn main() {
    let tuple = (10, "hello");

    match tuple {
        (_x, "hello") => println!("A tuple with the word hello"),
        (0, _) => println!("A tuple with value of zero"),
        _ => println!("A match case that we did not cover"),
    }
}