const INVENTORY_SIZE: usize = 9;
const WEIGHT_CAP: u8 = 50;
#[derive(Clone)]
struct Inventory(Vec<Item>);

#[derive(Clone, Eq, PartialEq)]
struct Item {
    name: &'static str,
    weight: u8,
}

fn main() {}
