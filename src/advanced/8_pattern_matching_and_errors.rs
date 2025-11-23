enum CustomEnum {
    Enum1,
    Enum2(u32),
    Enum3(String),
}

fn main() {
    let en = CustomEnum::Enum2(12);

    match en {
        CustomEnum::Enum1 => println!("A Type of Enum1"),
        CustomEnum::Enum2(number) => println!("A Type of Enum2({})", number),
        CustomEnum::Enum3(word) => println!("A Type of Enum3: {}", word),
    }
}