fn main() {
    let a: char = 'A';

    {
        let a: char = 'a';

        println!("{}", a);
    }

    println!("{}", a);

    println!("{}", helper(true));
    println!("{}", helper(false));
}

fn helper(fact: bool) -> String {
    if fact {
        String::from("Truth")
    } else {
        String::from("Lie")
    }
}