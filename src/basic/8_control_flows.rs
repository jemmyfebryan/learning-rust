fn main() {
    let number = 42;

    if number % 2 == 0 {
        println!("{} is even", number);

        if number % 3 == 0 {
            println!("{} is divisible by 3.", number);
        } else {
            println!("{} is not divisible by 3.", number);
        }
    } else {
        println!("{} is odd.", number);

        if number % 5 == 0 {
            println!("{} is divisible by 5.", number);
        } else {
            println!("{} is not divisible by 5.", number);
        }
    }

    if number % 2 == 0 && number % 3 == 0 {
        println!("{} is an even number that is divisible by 3.", number)
    }
}