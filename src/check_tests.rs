use std::collections::HashMap;

use crate::ast::Process;
use crate::ast::Simulation;
use crate::ast::Resource;
use crate::check::check;
use crate::check::Output;

#[test]
fn happy_path() {

	let mut inventory = HashMap::new();
	inventory.insert(String::from("wood"), 10);

	let build_premium_process = Process {
		name: String::from("build_premium"),
		input: vec!(Resource {name: String::from("wood"), quantity: 3}),
		output: vec!(Resource {name: String::from("premium_chair"), quantity: 1}),
		duration: 40,
		h: 0
	};

	let build_standard_process = Process {
		name: String::from("build_standard"),
		input: vec!(Resource {name: String::from("wood"), quantity: 3}),
		output: vec!(Resource {name: String::from("standard_chair"), quantity: 1}),
		duration: 10,
		h: 0
	};

	let mut processes = HashMap::new();
	processes.insert(String::from("build_premium"), build_premium_process);
	processes.insert(String::from("build_standard"), build_standard_process);

	let mock_output = Output {
		steps: vec![
			String::from("build_premium"),
			String::from("build_standard"),
		],
	};

	let mock_simulation = Simulation {
		inventory,
		processes,
		optimize: vec!(String::from("premium_chair")),
		optimize_time: false,
	};

	let expected_inventory = hashmap!(
		"wood".to_string() => 4,
		"standard_chair".to_string() => 1,
		"premium_chair".to_string() => 1,
	);
	let expected = Ok(expected_inventory);

	assert_eq!(check(mock_simulation, mock_output), expected);
}
