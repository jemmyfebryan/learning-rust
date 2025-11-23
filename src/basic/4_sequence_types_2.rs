fn main() {
    let num_arr_1: [u32; 5] = [0, 1, 2, 3, 4];
    // let num_arr_2: [u32; 5] = [0, 1, 2, 3, -4];
    // let num_arr_3: [u32; 5] = [0];

    for i in 0..num_arr_1.len() {
        println!("Element at index {}: {}", i, num_arr_1[i]);
    }

    for element in num_arr_1.iter() {
        println!("Element: {}", element)
    }

}