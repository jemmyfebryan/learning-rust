fn main () {
    // Write a short program that prints each number from 1 to 15 on a new line.
    // For each multiple of 3, print "Fizz".
    // For each multiple of 5, print "Buzz".
    // For numbers which are multiple of both 3 and 5, print "FizzBuzz".
    // If non of these statements are statisfied print out the number

    let n: u8 = 15;

    for i in 1..n+1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}