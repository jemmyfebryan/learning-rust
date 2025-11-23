fn main() {
    let array = [1, 2, 3, 4, 5];

    match &array[..] {
        [first, second, rest @ ..] => {
            println!("First: {}, Second: {}", first, second);
            println!("Rest: {:?}", rest);
        },
        _ => println!("A slice which we did not cover!"),
    }
}