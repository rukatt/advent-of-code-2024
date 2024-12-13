use std::fs::File;
use std::io::{self, BufRead};

fn parse_num_pair(line: String) -> (i32, i32) {
    let mut it = line.split("   ");
    let n1 = it.next().unwrap().parse::<i32>().unwrap();
    let n2 = it.next().unwrap().parse::<i32>().unwrap();
    return (n1, n2);
}

fn main() {
    let mut distance: i32 = 0;
    let mut similarity: i32 = 0;
    
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    let file = File::open("input.txt").expect("file not found");
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
	let line = line.expect("can't read file");
	let (n1, n2) = parse_num_pair(line);
	list1.push(n1);
	list2.push(n2);
    }
    assert!(list1.len() == list2.len());
    
    list1.sort();
    list2.sort();
    
    for i in 0..list1.len() {
	if list1[i] > list2[i] {
	    distance += list1[i] - list2[i];
	} else {
	    distance += list2[i] - list1[i];
	}
    }

    println!("the total distance between the two lists is {}", distance);

    for i in list1.iter() {
	for j in list2.iter() {
	    if i == j {
		similarity += i;
	    }
	}
    }

    println!("the similarity score of the two lists is {}", similarity);
}
