use std::collections::HashMap;

mod check;
mod ast;

fn main() {
	let mock_output = check::Output {
		steps: vec![
			String::from("build_premium"),
			String::from("build_standard"),
		],
	};

	let mut inventory = HashMap::new();
	inventory.insert(String::from("wood"), 10);

	let build_premium_process = ast::Process {
		name: String::from("build_premium"),
		input: vec!(ast::Resource {name: String::from("wood"), quantity: 3}),
		output: vec!(ast::Resource {name: String::from("premium_chair"), quantity: 1}),
		duration: 40,
		h: 0
	};

	let build_standard_process = ast::Process {
		name: String::from("build_standard"),
		input: vec!(ast::Resource {name: String::from("wood"), quantity: 3}),
		output: vec!(ast::Resource {name: String::from("standard_chair"), quantity: 1}),
		duration: 10,
		h: 0
	};

	let mut processes = HashMap::new();
	processes.insert(String::from("build_premium"), build_premium_process);
	processes.insert(String::from("build_standard"), build_standard_process);

	let mock_simulation = ast::Simulation {
		inventory,
		processes,
		optimize: vec!(String::from("premium_chair")),
		optimize_time: false,
	};

	println!("Running simulation with this initial stock:");
	ast::display_inventory(&mock_simulation.inventory);
	println!("\nRunning...\n");
	match check::check(mock_simulation, mock_output) {
		Err (err) => {
			println!("An error occured: {}", err);
		}
		Ok (updated_inventory) => {
			println!("Simulation ended successfully with this stock:");
			ast::display_inventory(&updated_inventory);
		}
	}
}
