use std::ops::Add;

fn main() {
    // fn max<T: PartialOrd>(x: T, y: T) -> T {
    //     if x >= y {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // let x: u64 = 10;
    // let y: u64 = 2;
    // let result = max(x, y);

    // println!("{}", result)




    fn add<T: Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }

    let x: i16 = 10;
    let y: i16 = 2;

    let result = add(x, y);

    println!("{}", result)
}