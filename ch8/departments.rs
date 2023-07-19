use std::io;
use std::collections::HashMap;

pub fn run_interface() {
    let mut depts: HashMap<String, Vec<String>> = HashMap::new();
    loop {
	let mut request = String::new();
	io::stdin()
	    .read_line(&mut request)
	    .expect("Failed to read request");
	let trimmed = request.trim();

	if trimmed == "ls" {
	    println!("Departments:");
	    for (key, value) in &depts {
		println!("Department: {key}, Employees: {value:?}");
		continue;
	    }
	} else if trimmed == "quit" {
	    println!("exiting");
	    break;
	} else {
	    let parts: Vec<&str> = trimmed.split(' ').collect();
	    if parts.len() != 4 {
		continue;
	    }
	    let employee = parts[1]; // type: &str
	    let dept = parts[3];
	    let existing = depts.entry(String::from(dept)).or_insert_with(Vec::new);
	    existing.push(employee.to_owned());
	    existing.sort();
	}
    }
}
