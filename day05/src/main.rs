#![allow(unused_parens)]

use std::fs;
use std::path::Path;

#[cfg(windows)]
const LINE_ENDING : &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &'static str = "\n";

struct InputData {
    page_ordering_rules: Vec<(u32, u32)>,
    pages_to_produce: Vec<Vec<u32>>,
}

// Prelude

fn parse_input_data(fstr: &String, dest: &mut InputData) {
    let lines: Vec<&str> = fstr.split(LINE_ENDING).collect();
    
    for l in lines {
	// step 1: parse the tuples/orderings
	if l.contains("|") {
	    let mut spl = l.split("|");
	    
	    let n1 = match spl.next() {
		Some(s) => s.parse::<u32>().unwrap(),
		None => panic!(),
	    };
	    let n2 = match spl.next() {
		Some(s) => s.parse::<u32>().unwrap(),
		None => panic!(),
	    };
	    dest.page_ordering_rules.push((n1,n2));
	}
	// step 2: parse the lists of numbers at the end
	else if l.contains(",") {
	    let mut lst: Vec<u32> = vec![];
	    let spl = l.split(",");

	    for n in spl {
		lst.push(n.parse::<u32>().unwrap());
	    }
	    dest.pages_to_produce.push(lst);
	}
    }
}

// Part 1

fn _part_1() {
    
}

fn main() {
    let path = "input.txt";
    let file_as_str = match fs::read_to_string(Path::new(path)) {
	Ok(s) => s,
	Err(_) => panic!("Can't read {}", path),
    };

    let mut parsed_input = InputData {
	page_ordering_rules: vec![],
	pages_to_produce: vec![],
    };

    parse_input_data(&file_as_str, &mut parsed_input);

}
