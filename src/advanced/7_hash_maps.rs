use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u32, String> = HashMap::new();

    let keys: [u32; 5] = [1, 2, 3, 4, 5];
    let values: [String; 5] = [
        String::from("Marko"),
        String::from("Polo"),
        String::from("Zack"),
        String::from("Marko"),
        String::from("Mike"),
    ];

    for i in 0..keys.len() {
        map.insert(keys[i], values[i].clone());
    }

    for i in map.iter(){
        println!("{}: {}", i.0, i.1)
    }

    let element = map.get(&1).unwrap();

    println!("{}", element);

    map.remove(&3);

    for i in map.iter(){
        println!("{}: {}", i.0, i.1)
    };

    for i in map.keys(){
        println!("{}: {}", i, map[i])
    };

    for i in map.values(){
        println!("{}", i)
    };
}