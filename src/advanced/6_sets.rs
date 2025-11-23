use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    set.insert(1);
    set.remove(&1);

    let contains = set.contains(&2);

    for element in &set {
        println!("{}", element)
    }

    println!("{}", contains);
}