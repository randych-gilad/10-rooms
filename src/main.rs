use std::io::{stdin, stdout, Write};

const INVENTORY_SIZE: usize = 9;
const WEIGHT_CAP: u8 = 50;
#[derive(Clone)]
struct Inventory(Vec<Item>);

#[derive(Clone, Eq, PartialEq)]
struct Item {
  name: String,
  weight: u8,
  attack: Option<u8>,
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
  loadout: Loadout,
}
struct Loadout {
  rhand: Option<Item>,
}
impl Loadout {
  fn check(&self) {
    match self.rhand.as_ref() {
      Some(val) => println!("Right hand: {}", val.name),
      None => println!("Nothing equipped."),
    }
  }
}
trait Attack<T> {
  fn attack(&self, target: &mut T);
}
impl Attack<Enemy> for Player {
  fn attack(&self, target: &mut Enemy) {
    // target.hp -= self.loadout.rhand.as_ref().unwrap().attack.unwrap();
    match self.loadout.rhand.as_ref() {
      Some(weapon) => target.hp -= weapon.attack.unwrap(),
      None => target.hp -= 1,
    }
  }
}
impl Attack<Player> for Enemy {
  fn attack(&self, target: &mut Player) {
    target.hp -= 1;
  }
}

struct Rooms(Vec<Room>);

impl Rooms {
  fn populate(&mut self) {
    self.0.push(Room {
      enemy: None,
      loot: Some(Item {
        name: "Wooden Sword".into(),
        weight: 1,
        attack: Some(1),
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
        attack: Some(1),
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
  fn remove_enemy(&mut self) {
    self.enemy = None
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
    self.0.remove(index);
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
    loadout: Loadout { rhand: None },
  };
  let rooms_ref = &mut rooms;
  let mut input = String::new();
  loop {
    println!("General commands -> q: Exit | w: Move/Attack | e: Inspect room");
    println!("Inventory -> r: Equip | t: Pick up | i: Inventory | c: Capacity | x: Loadout");
    println!("Name: {} | HP: {}", player.name, player.hp);
    print!("Your command: ");
    stdout().flush().unwrap();
    match stdin().read_line(&mut input) {
      Ok(_) => match input.trim() {
        "q" => {
          clear_screen();
          break;
        }
        "w" => match rooms_ref.room().enemy.as_mut() {
          Some(enemy) => {
            println!("You hit {} for 1 attack.", enemy.name);
            player.attack(enemy);
            println!("{} took 1 damage. {} HP left.", enemy.name, enemy.hp);
            if enemy.hp == 0 {
              println!("{} died.", enemy.name);
              rooms_ref.room().remove_enemy()
            }
          }
          None => {
            rooms_ref.move_room();
            println!("Room changed.");
            rooms_ref.room().look()
          }
        },
        "e" => rooms_ref.room().look(),
        "r" => match player.inventory.0.len() {
          0 => println!("Cannot equip. Inventory is empty."),
          _ => {
            player.loadout.rhand = Some(player.inventory.0.remove(0));
          }
        },
        "t" => {
          player
            .inventory
            .add_item(rooms_ref.room().loot.clone().unwrap());
          rooms_ref.room().remove_item();
        }
        "i" => player.inventory.list_inventory(),
        "c" => player.inventory.check_capacity(),
        "x" => player.loadout.check(),
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
