fn main() {
    // let vec_1: Vec<u32> = Vec::new();
    let mut vec: Vec<u32> = vec![1, 2, 3, 4, 5];

    // for element in vec.iter() {
    //     println!("{}", element)
    // }

    // vec[0] = 0;

    // for element in vec.iter() {
    //     println!("{}", element)
    // }



    // vec.push(6);

    // for element in vec.iter() {
    //     println!("{}", element)
    // }



    let popped = vec.pop().unwrap();

    println!("{}", popped);

    let popped = vec.pop().unwrap();

    println!("{}", popped);

    println!("{}", vec.len());
}