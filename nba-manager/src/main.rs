use std::collections::HashMap;
use std::io::{self, Read};
use std::thread::current;

struct Player {
    // player name
    name: String,
    room_name: String,
    health: isize,
    inventory: Vec<Item>,
}

struct Room {
    name: String, // possiblyredundant
    items: Vec<Item>,
    require_keycard: bool,
}

struct Item {
    name: String,
    health_effect: isize,
}

fn main() {
    
    // initialize state
    let mut room_list: HashMap<String, Room> = HashMap::new();
    // initial states:
    let ball = Item {
        name: "ball".to_string(),
        health_effect: 5,
    };

    // Items (mutability not needed since we don't need to change items)
    let towel = Item {
        name: "Towel".to_string(),
        health_effect: 50,
    };

    let lock = Item {
        name: "Lock".to_string(),
        health_effect: -5,
    };

    let keycard = Item {
        name: "Keycard".to_string(),
        health_effect: 0
    };

    // Rooms (mutability not needed since we put it in mutable hashmap)
    let lobby = Room {
        name: "Lobby".to_string(),
        items: vec![],
        require_keycard: false,
    };

    let court = Room {
        name: "Court".to_string(),
        items: vec![ball],
        require_keycard: false,
    };

    let locker_room = Room {
        name: String::from("Locker Room"),
        items: vec![towel, lock],
        require_keycard: true,
    };

    room_list.insert(String::from("Lobby"), lobby);
    room_list.insert(String::from("Court"), court);
    room_list.insert(String::from("Locker Room"), locker_room);

    let mut my_player = Player {
        name: String::from("None"), // ask to change String later
        room_name: String::from("Start"),
        health: 100,
        inventory: Vec::new(),
    };

    // start loop
    println!("Hey, before we start... what is your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    my_player.name = name.trim().to_string(); // @TODO capitalize first letter
    my_player.room_name = "Lobby".to_string();
    let mut exited = false;
    println!("Hi {}! Welcome to NBA Manager!", my_player.name);


    while !exited {
        println!("\nYou're currently in the {}", my_player.room_name);
        println!("What would like to do? (Choose a letter)");
        println!("Your options are: A. View inventory, B. View items in the room, C. Change rooms, D. Check health, E. Exit");
        let mut choice: String = String::from("");
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        choice = choice.trim().to_string().to_lowercase();
        if choice.len() != 1 {
            println!("Try again, just try the letter");
        }
        
        if choice == "a" { 
            println!("Here are the inventory items:");
            print_items(&my_player.inventory);
        } else if choice == "b" {
            println!("Here are the room items:");
            let room = room_list.get(&my_player.room_name).expect("Could not find room");
            print_items(&room.items);

            println!("What would you like to do? A. Pick up an item, B. Place item from inventory, C. ")
        } else if choice == "c" {
            println!("Hi {}! what room would you like to enter?", my_player.name);
            println!("Choose from the following:");
            print_room_choices(&room_list, &my_player.room_name);
            let mut room_choice = String::new();
            io::stdin().read_line(&mut room_choice).expect("Failed to read line");
            println!("attempting entry...");
            room_choice = room_choice.trim().to_string();
            match attempt_entry(&mut my_player, &room_choice, &room_list) {
                Ok(val) => println!("{val}"),
                Err(e) => {
                    println!("Error: {e}");
                },
            }
            println!("Ok, you are now entering room: {room_choice}");
        } else if choice == "d" {
            println!("Your health is at: {}", my_player.health);
        } else if choice == "e" {
            exited = true;
        } else {
            println!("Invalid choice, try again")
        }
    }


    // choices: pick up item, drop item, or leave room

}

fn print_room_choices(room_list: &HashMap<String, Room>, current_room: &String) {
    // room_list
    for room in room_list.keys() {
        if room == current_room {
            println!("- {room} <-- current here");
        } else {
            println!("- {room} ");
        }

    }
}

fn print_items(item_list: &Vec<Item>) {
    for item in item_list {
        println!("-{}", item.name);
    }
}

fn attempt_entry (player: &mut Player, room_name: &String, room_list: &HashMap<String, Room>) -> Result<String, String> {
    if room_list.contains_key(room_name) {
        // @TODO add keycard check
        if player.room_name == *room_name {
            Ok("Player is already in the room!".to_string())
        } else {
            player.room_name = room_name.clone();
            Ok("Entry successful!".to_string())
        }
    } else {
        Err("Unable to enter, room does not exist".to_string())
    }
}