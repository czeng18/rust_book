use std::collections::HashMap;

pub fn find_stats(lst: &Vec<i32>) -> (i32, i32) {
    let mut sorted = lst.clone();
    sorted.sort();
    let median = sorted[lst.len() / 2];
    let median_index = lst.iter().position(|&r| r == median).unwrap();
    
    let mut values = HashMap::new();
    for i in lst {
	let count = values.entry(i).or_insert(0);
	*count += 1;
    }

    let mut max_count = 0;
    let mut mode = 0;
    
    for (key, value) in values {
	if value > max_count {
	    max_count = value;
	    mode = *key;
	}
    }

    let mode_index = lst.iter().position(|&r| r == mode).unwrap();

    (lst[median_index], lst[mode_index])
}
