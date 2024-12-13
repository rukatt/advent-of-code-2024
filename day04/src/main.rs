#![allow(unused_parens)]

use std::fs;
use std::path::Path;

#[cfg(windows)]
const LINE_ENDING : &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING : &'static str = "\n";

// Part 1

// We are going to process the file line by line, even if it leads to a stupid algorithm

// step 1: load the file in RAM as a matrix
fn fill_matrix_from_fstr(fstr: &String, v: &mut Vec<String>) {
    let lines = fstr.split(LINE_ENDING);
    for l in lines {
	if l.len() > 0 {
	    v.push(l.to_string());
	}
    }
}


// step 2: calculate the horizontal occurences
fn get_horizontal_xmases(line: &String) -> usize {
    let spv: Vec<&str> = line.split("XMAS").collect();
    let vps: Vec<&str> = line.split("SAMX").collect();
    spv.len() - 1 + vps.len() - 1
}


// step 3: calculate the vertical occurences
fn get_vertical_xmases(mat: &Vec<String>) -> usize {
    let mut acc = 0;
    
    for (i, line) in mat.into_iter().enumerate() {
	for (j, ch) in line.chars().into_iter().enumerate() {
	    // first, we look upward
	    if ch == 'X' && i as i64 - 3 >= 0 {
		if (mat[i-1].chars().nth(j).unwrap() == 'M'
		    && mat[i-2].chars().nth(j).unwrap()== 'A'
		    && mat[i-3].chars().nth(j).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	    // then downwards
	    if ch == 'X' && i + 3 < mat.len() {
		if (mat[i+1].chars().nth(j).unwrap() == 'M'
		    && mat[i+2].chars().nth(j).unwrap()== 'A'
		    && mat[i+3].chars().nth(j).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	}
    }
    acc
}


// step 4: calculate the diagonal occurences
fn get_diagonal_xmases(mat: &Vec<String>) -> usize {
    let mut acc = 0;
    
    for (i, line) in mat.into_iter().enumerate() {
	for (j, ch) in line.chars().into_iter().enumerate() {
	    // northwest
	    if ch == 'X' && i as i64 - 3 >= 0 && j as i64 - 3 >= 0 {
		if (mat[i-1].chars().nth(j-1).unwrap() == 'M'
		    && mat[i-2].chars().nth(j-2).unwrap()== 'A'
		    && mat[i-3].chars().nth(j-3).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	    // northeast
	    if ch == 'X' && i as i64 - 3 >= 0 && j + 3 < mat.len() {
		if (mat[i-1].chars().nth(j+1).unwrap() == 'M'
		    && mat[i-2].chars().nth(j+2).unwrap()== 'A'
		    && mat[i-3].chars().nth(j+3).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	    // southeast
	    if ch == 'X' && i + 3 < mat.len() && j + 3 < mat.len() {
		if (mat[i+1].chars().nth(j+1).unwrap() == 'M'
		    && mat[i+2].chars().nth(j+2).unwrap()== 'A'
		    && mat[i+3].chars().nth(j+3).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	    // southwest
	    if ch == 'X' && i + 3 < mat.len() && j as i64 - 3 >= 0 {
		if (mat[i+1].chars().nth(j-1).unwrap() == 'M'
		    && mat[i+2].chars().nth(j-2).unwrap()== 'A'
		    && mat[i+3].chars().nth(j-3).unwrap() == 'S') {
		    acc += 1;
		}
	    }
	}
    }
    acc
}


fn part1(matrix: &Vec<String>) {
    let mut h_xmases = 0;
    for line in matrix {
	h_xmases += get_horizontal_xmases(&line);
    }

    let v_xmases = get_vertical_xmases(matrix);
    let d_xmases = get_diagonal_xmases(matrix);
    
    let sum = h_xmases + v_xmases + d_xmases;
    println!("Part 1 => {}", sum);
}


// Part 2

fn get_2d_xmases(mat: &Vec<String>) -> usize {
    let mut acc = 0;
    
    for (i, line) in mat.into_iter().enumerate() {
	for (j, ch) in line.chars().into_iter().enumerate() {
	    if (ch == 'A' &&
		i as i64 - 1 >= 0 &&
		j as i64 - 1 >= 0 &&
		i + 1 < mat.len() &&
		j + 1 < mat.len()
	    ) {
		// check north west -> south east
		if !((mat[i-1].chars().nth(j-1).unwrap() == 'M' &&
		      mat[i+1].chars().nth(j+1).unwrap() == 'S') ||
		     (mat[i-1].chars().nth(j-1).unwrap() == 'S' &&
		      mat[i+1].chars().nth(j+1).unwrap() == 'M')) {
		    continue;
		   }
		// and south west -> north east 
		if ((mat[i+1].chars().nth(j-1).unwrap() == 'M' &&
		     mat[i-1].chars().nth(j+1).unwrap() == 'S') ||
		    (mat[i+1].chars().nth(j-1).unwrap() == 'S' &&
		     mat[i-1].chars().nth(j+1).unwrap() == 'M')) {
		       acc += 1;
		   }
	    }
	}
    }
    acc
}


fn part2(matrix: &Vec<String>) {
    println!("Part 2 => {}", get_2d_xmases(matrix));
}

// -----

fn main() {
    let path = "input.txt";
    let file_as_str = match fs::read_to_string(Path::new(path)) {
	Ok(s) => s,
	Err(_) => panic!("Can't read {}", path),
    };

    let mut matrix = vec![];
    fill_matrix_from_fstr(&file_as_str, &mut matrix);

    part1(&matrix);
    part2(&matrix);
}
