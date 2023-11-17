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
}
fn main() {}
