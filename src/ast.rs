pub use std::collections::HashMap;

pub type Inventory = HashMap<String, usize>;

#[derive(Clone)]
pub struct Resource {
	pub name: String,
	pub quantity: usize,
}

pub struct Process {
    pub name: String,
    pub input: Vec<Resource>,
    pub output: Vec<Resource>,
    pub duration: i32,
    pub h: i32,
}

pub struct Simulation {
    pub inventory: Inventory,
    pub processes: HashMap<String, Process>,
    pub optimize: Vec<String>,
    pub optimize_time: bool,
}

pub fn display_inventory(inventory: &Inventory) {
	for key in inventory.keys() {
		match inventory.get(key) {
			Some (value) => {
				println!("{}: {}", key, value);
			}
			None => {
				println!("{}: ?", key);
			}
		}
	}
}
