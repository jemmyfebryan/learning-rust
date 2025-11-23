fn main() {
    let date: (u32, u32, u32, String) = get_date(String::from("12-12-2020 GMT+1"));

    let (day, month, year, timezone) = date.clone();

    println!("{}, {}, {}, {}", day, month, year, timezone);
    println!("{}, {}, {}, {}", date.0, date.1, date.2, date.3);

    let our_date: (u32, u32, u32, String) = (12, 12, 2021, String::from("GMT+4"));

    println!(
        "{}, {}, {}, {}",
        our_date.0, our_date.1, our_date.2, our_date.3
    )
}

fn get_date(date: String) -> (u32, u32, u32, String) {
    let day: u32 = date[0..2].parse::<u32>().ok().unwrap();
    let month: u32 = date[3..5].parse::<u32>().ok().unwrap();
    let year: u32 = date[6..10].parse::<u32>().ok().unwrap();
    let timezone: String = String::from(&date[11..]);

    (day, month, year, timezone)
}