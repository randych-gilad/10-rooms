use std::io::{stdin, stdout, Write};

const INVENTORY_SIZE: usize = 9;
const WEIGHT_CAP: u8 = 50;
#[derive(Clone)]
struct Inventory(Vec<Item>);

#[derive(Clone, Eq, PartialEq)]
struct Item {
  name: &'static str,
  weight: u8,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Enemy<'a> {
  name: &'a str,
  hp: u8,
}

struct Player<'a> {
  name: &'a str,
  hp: u8,
  inventory: Inventory,
  current_room: u8,
}

struct Rooms<'a>(Vec<Room<'a>>);

impl<'a> Rooms<'a> {
  fn look(&self) {
    if let Some(room) = self.0.get(0) {
      room.look();
    }
  }
}

#[allow(dead_code)]
struct Room<'a> {
  enemy: Option<Enemy<'a>>,
  loot: Option<Item>,
}
impl<'a> Room<'a> {
  fn look(&self) {
    if let Some(val) = &self.enemy {
      println!("You see enemies: {:?}", val)
    }
    if let Some(val) = &self.loot {
      println!("You see loot: {} ({} kg)", val.name, val.weight)
    }
  }
}
impl<'a> Rooms<'a> {
  fn populate(&mut self) {
    self.0.push(Room {
      enemy: None,
      loot: Some(Item {
        name: "Wooden Sword",
        weight: 1,
      }),
    });
    self.0.push(Room {
      enemy: Some(Enemy {
        name: "Quack",
        hp: 1,
      }),
      loot: None,
    });
    self.0.push(Room {
      enemy: Some(Enemy {
        name: "Quack",
        hp: 1,
      }),
      loot: None,
    });
    self.0.push(Room {
      enemy: None,
      loot: Some(Item {
        name: "Beaned Sword",
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
        name: "Massive man",
        hp: 10,
      }),
      loot: None,
    });
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
  let mut inventory = Inventory(Vec::with_capacity(INVENTORY_SIZE));
  let mut input = String::new();
  loop {
    println!("q: Exit | w: move/attack | e: look around | i: inventory");
    print!("Your command: ");
    stdout().flush().unwrap();

    match stdin().read_line(&mut input) {
      Ok(_) => match input.trim() {
        "q" => {
          clear_screen();
          break;
        }
        "w" => todo!(),
        "e" => rooms.look(),
        "i" => inventory.list_inventory(),
        &_ => println!("You entered: {}", input.trim()),
      },
      Err(error) => {
        eprintln!("Error reading input: {}", error);
        break;
      }
    }
    input.clear();
  }
  // inventory.add_item(Item {
  //     name: "The needful",
  //     weight: 0,
  // });
  // inventory.add_item(Item {
  //     name: "Willy Wonka",
  //     weight: 60,
  // });
  // inventory.add_item(Item {
  //     name: "Web scrap",
  //     weight: 20,
  // });
  // inventory.add_item(Item {
  //     name: "Strawman Builder Kit",
  //     weight: 40,
  // });
  // inventory.add_item(Item {
  //     name: "Kangaroo emblem",
  //     weight: 1,
  // });
  // inventory.add_item(Item {
  //     name: "Peppermint fountain",
  //     weight: 10,
  // });
  // inventory.add_item(Item {
  //     name: "Whopper",
  //     weight: 0,
  // });
  // inventory.add_item(Item {
  //     name: "A tasty burger",
  //     weight: 1,
  // });
  // inventory.add_item(Item {
  //     name: "Haystack needle",
  //     weight: 1,
  // });
  // inventory.add_item(Item {
  //     name: "Cuphead master CD",
  //     weight: 2,
  // });
  // inventory.add_item(Item {
  //     name: "A taste of bright future",
  //     weight: 10,
  // });
  // inventory.list_inventory();
  // inventory.drop_item(3);
  // inventory.add_item(Item {
  //     name: "A taste of bright future",
  //     weight: 10,
  // });
  // inventory.list_inventory();
  // inventory.check_capacity();
}
