#![allow(unused_parens)]

use std::fs::File;
use std::io::{self, BufRead};

fn parse_levels() -> Vec<Vec<i32>> {
    let mut levels: Vec<Vec<i32>> = Vec::new();
    
    let file = File::open("input.txt").expect("Input file not found");
    let reader = io::BufReader::new(file);

    let mut i: usize = 0;
    for line in reader.lines() {	
	let line = line.expect("Can't read input file");
	let strnums = line.split(" ");

	levels.push(Vec::new());
	
	for strnum in strnums {
	    levels[i].push(strnum.parse::<i32>().unwrap());
	}
	i += 1;
    }
    return levels;
}

fn is_safe(report: Vec<i32>) -> bool {
    let mut is_increasing: bool = false;
    let mut is_decreasing: bool = false;

    for i in 1..report.len() {
	if (report[i-1] == report[i] ||
	    report[i-1] - report[i] > 3 ||
	    report[i] - report[i-1] > 3) {
	    return false;
	}
	if report[i] > report[i-1] {
	    is_increasing = true;
	}
	if report[i-1] > report[i] {
	    is_decreasing = true;
	}
    }
    return is_increasing ^ is_decreasing;
}

// Part 2

fn generate_dampened_reports(reports: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut drs = Vec::new();

    for (i, report) in reports.iter().enumerate() {
	drs.push(Vec::new());
	for j in 0..report.len() {
	    drs[i].push(report.clone());
	    drs[i][j].remove(j);
	}
    }
    return drs;
}

fn are_dampened_possibilities_safe(drs: Vec<Vec<i32>>) -> bool {
    for hypothetical_report in drs {
	if (is_safe(hypothetical_report)) {
	    return true;
	}
    }
    return false;
}


fn main() {
    let mut nb_safe: i32 = 0;
    let reports = parse_levels();

    for rep in &reports {
        if is_safe(rep.to_vec()) {
	       nb_safe += 1;
	   }
    }
    println!("{} reports are safe without dampening", nb_safe);
    
    nb_safe = 0;

    let damp_reports = generate_dampened_reports(reports);

    for rep in &damp_reports {
	if are_dampened_possibilities_safe(rep.to_vec()) {
	    nb_safe += 1;
	}
    }
    println!("{} reports are safe with dampening", nb_safe);

}
