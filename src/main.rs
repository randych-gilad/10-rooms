use std::io::{stdin, stdout, Write};

const INVENTORY_SIZE: usize = 9;
const WEIGHT_CAP: u8 = 50;
#[derive(Clone)]
struct Inventory(Vec<Item>);

#[derive(Clone, Eq, PartialEq)]
struct Item {
  name: String,
  weight: u8,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Enemy {
  name: String,
  hp: u8,
}

struct Player {
  name: String,
  hp: u8,
  inventory: Inventory,
}

struct Rooms(Vec<Room>);

impl Rooms {
  fn populate(&mut self) {
    self.0.push(Room {
      enemy: None,
      loot: Some(Item {
        name: "Wooden Sword".into(),
        weight: 1,
      }),
    });
    self.0.push(Room {
      enemy: Some(Enemy {
        name: "Quack".into(),
        hp: 1,
      }),
      loot: None,
    });
    self.0.push(Room {
      enemy: Some(Enemy {
        name: "Quack".into(),
        hp: 1,
      }),
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: Some(Item {
        name: "Beaned Sword".into(),
        weight: 2,
      }),
    });
    self.0.push(Room {
      enemy: None,
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: None,
    });
    self.0.push(Room {
      enemy: Some(Enemy {
        name: "Massive man".into(),
        hp: 10,
      }),
      loot: None,
    });
  }
  fn room(&mut self) -> &mut Room {
    self.0.get_mut(0).unwrap()
  }
  fn move_room(&mut self) {
    self.0.remove(0);
  }
}

impl Rooms {}

#[allow(dead_code)]
struct Room {
  enemy: Option<Enemy>,
  loot: Option<Item>,
}
impl Room {
  fn look(&self) {
    match (&self.enemy, &self.loot) {
      (Some(enemy), Some(loot)) => {
        println!("You see enemies: {:?}", enemy);
        println!("You see loot: {} ({} kg)", loot.name, loot.weight);
      }
      (Some(enemy), None) => println!("You see enemies: {:?}", enemy),
      (None, Some(loot)) => println!("You see loot: {} ({} kg)", loot.name, loot.weight),
      (None, None) => println!("You see nothing of interest."),
    }
  }
  fn remove_item(&mut self) {
    self.loot = None
  }
}

impl Inventory {
  fn add_item(&mut self, item: Item) {
    match self.0.len() {
      ..=INVENTORY_SIZE => {
        println!("Added {} at slot {}.", item.name, self.0.len() + 1);
        self.0.push(item);
      }
      _ => println!("Cannot add {}. Inventory is full.", item.name),
    }
  }
  fn list_inventory(&self) {
    println!("\nItems in inventory:");
    self.0.iter().enumerate().for_each(|(index, item)| {
      println!("[{}]: {}, {} kg.", index + 1, item.name, item.weight);
    });
    println!();
  }
  fn drop_item(&mut self, index: usize) {
    self.0.remove(index - 1);
  }
  fn check_capacity(&self) {
    let total_weight: u8 = self.0.iter().map(|item| item.weight).sum();
    if total_weight > WEIGHT_CAP {
      println!("Weight exceeded by {} kg.", total_weight - WEIGHT_CAP);
      println!("Obvious items to drop:\n");
      let mut sorted = self.0.clone();
      sorted.sort_by(|original, sorted| sorted.weight.cmp(&original.weight));
      sorted
        .iter()
        .filter(|item| item.weight >= WEIGHT_CAP / 2)
        .for_each(|item| println!("{}, {} kg", item.name, item.weight));
    } else {
      println!("Can carry {} more kg.", WEIGHT_CAP - total_weight)
    }
  }
}
fn clear_screen() {
  if cfg!(windows) {
    print!("{}c", 27 as char);
  } else {
    print!("\x1B[2J\x1B[H");
  }
}
fn main() {
  let mut rooms: Rooms = Rooms(Vec::new());
  rooms.populate();
  let mut player: Player = Player {
    name: "Brave".into(),
    hp: 10,
    inventory: Inventory(Vec::with_capacity(INVENTORY_SIZE)),
  };
  let rooms_ref = &mut rooms;
  let mut input = String::new();
  loop {
    println!("q: Exit | w: move/attack | e: look around | t: pick up | i: inventory | c: capacity");
    println!("Name: {} | HP: {}", player.name, player.hp);
    print!("Your command: ");
    stdout().flush().unwrap();
    match stdin().read_line(&mut input) {
      Ok(_) => match input.trim() {
        "q" => {
          clear_screen();
          break;
        }
        "w" => {
          rooms_ref.move_room();
          println!("Room changed.");
        }
        "e" => rooms_ref.room().look(),
        "t" => {
          player
            .inventory
            .add_item(rooms_ref.room().loot.clone().unwrap());
          rooms_ref.room().remove_item();
        }
        "i" => player.inventory.list_inventory(),
        "c" => player.inventory.check_capacity(),
        &_ => println!("You entered: {}", input.trim()),
      },
      Err(error) => {
        eprintln!("Error reading input: {}", error);
        break;
      }
    }
    input.clear();
    println!("------------------------------------")
  }
}
