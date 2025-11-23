struct Surfer {
    pub height: u32,
    pub weight: u32,
    pub max_wave_height: u32,
    pub board_name: String,
}

impl Surfer {
    fn say_aloha(&self) {
        println!("Alohaaaa!");
    }

    fn ride_wave(&mut self, wave_height: u32) {
        if wave_height > self.max_wave_height {
            self.max_wave_height = wave_height;
            println!("Wow, that's a new record!");
        } else {
            println!("That was awesome!");
        }
    }

    fn change_board_name(&mut self, new_name: String) {
        self.board_name = new_name;
    }
}

fn main() {
    let mut surfer = Surfer {
        height: 188,
        weight: 77,
        board_name: String::from("Blocky"),
        max_wave_height: 0,
    };

    println!(
        "Height: {}, weight: {}, favorite board name: {}, biggest wave: {}",
        surfer.height, surfer.weight, surfer.board_name, surfer.max_wave_height
    );

    surfer.say_aloha();

    let old_board = surfer.board_name.clone();
    surfer.change_board_name(String::from("Cryptic"));
    println!(
        "old board name: {}, new board name: {}",
        old_board, surfer.board_name
    );

    let previous_max_wave = surfer.max_wave_height.clone();
    surfer.ride_wave(5);
    println!(
        "Previous wave height recorded: {}, new have height record: {}",
        previous_max_wave, surfer.max_wave_height
    );

    surfer.ride_wave(2);

}