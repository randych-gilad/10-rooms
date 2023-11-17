const INVENTORY_SIZE: usize = 9;
const WEIGHT_CAP: u8 = 50;
#[derive(Clone)]
struct Inventory(Vec<Item>);

#[derive(Clone, Eq, PartialEq)]
struct Item {
    name: &'static str,
    weight: u8,
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
        let total_weight = self.0.iter().map(|item| item.weight).sum();
        match total_weight {
            ..=WEIGHT_CAP => {}
            _ => {
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
}
fn main() {
    let mut inventory = Inventory(Vec::with_capacity(INVENTORY_SIZE));
    inventory.add_item(Item {
        name: "The needful",
        weight: 0,
    });
    inventory.add_item(Item {
        name: "Willy Wonka",
        weight: 60,
    });
    inventory.add_item(Item {
        name: "Web scrap",
        weight: 20,
    });
    inventory.add_item(Item {
        name: "Strawman Builder Kit",
        weight: 40,
    });
    inventory.add_item(Item {
        name: "Kangaroo emblem",
        weight: 1,
    });
    inventory.add_item(Item {
        name: "Peppermint fountain",
        weight: 10,
    });
    inventory.add_item(Item {
        name: "Whopper",
        weight: 0,
    });
    inventory.add_item(Item {
        name: "A tasty burger",
        weight: 1,
    });
    inventory.add_item(Item {
        name: "Haystack needle",
        weight: 1,
    });
    inventory.add_item(Item {
        name: "Cuphead master CD",
        weight: 2,
    });
    inventory.add_item(Item {
        name: "A taste of bright future",
        weight: 10,
    });
}
