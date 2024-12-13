use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

enum ReadError {
    EOF,
}

type NumPair = Result<(u32, u32), ReadError>;

// I want to only depend on the stdlib for this one, I'm too lazy to install a crate

// Part 1 -- Parsing with a buffered reader

fn get_next_mul_expr<R: BufRead + Seek>(reader: &mut R) -> NumPair {
    let mut buf = vec![];
    let mut strm_pos;

    // Go to the first 'm' in the character stream
    let nb_chars = reader.read_until(b'm', &mut buf).unwrap();
    
    // If the reader is empty, raise an error, else record the position in the stream
    if nb_chars == 0 {
	return Err(ReadError::EOF);
    }
    strm_pos = reader.stream_position().unwrap();

    // Check if we reached a 'mul(' token
    buf = vec![0u8; 3];
    let _ = reader.read_exact(&mut buf);
    let tokmul = String::from_utf8(buf).unwrap();
    
    if tokmul != "ul(" {
	let _ = reader.seek(SeekFrom::Start(strm_pos)); // rewind the tape
	return Ok((0,0));
    }

    strm_pos = reader.stream_position().unwrap();

    // Check if the token to parse is well formed
    buf = vec![0u8; 8];
    let _ = reader.read_exact(&mut buf);
    let _ = reader.seek(SeekFrom::Start(strm_pos)); // rewind for the next parsing
    
    let tokmul = String::from_utf8(buf).unwrap();
    if !tokmul.contains(",") && !tokmul.contains(")") {
	return Ok((0,0));
    }

    // Parse the numbers
    let mut n1: u32 = 0;
    let mut n2: u32 = 0;
    let mut n1_was_parsed = false;
    
    for c in tokmul.chars() {
	if !n1_was_parsed && c.is_digit(10) && n1 < 1000 {
	    n1 = n1 * 10 + c.to_digit(10).unwrap();
	}
	else if n1_was_parsed && c.is_digit(10) && n2 < 1000 {
	    n2 = n2 * 10 + c.to_digit(10).unwrap();
	}
	else if c == ',' && !n1_was_parsed {
	    n1_was_parsed = true;
	}
	else if c == ')' {
	    break;
	}
	else {
	    return Ok((0,0));
	}
    }
    
    Ok((n1,n2))
}

fn part1<R: BufRead + Seek>(reader: &mut R) {
    let mut acc: u64 = 0;
    loop {
	let t = match get_next_mul_expr(reader) {
	    Err(_) => {
		println!("Part 1 => {}", acc);
		return;
	    },
	    Ok(tuple) => tuple,
	};
	acc += t.0 as u64 * t.1 as u64;
    }
}

// Part 2 -- Ok, that was painful, let's load the whole file into a String struct now

fn part2(path: &str) -> Result<(), Box<dyn Error>> {
    let mut acc = 0;
    
    let file_as_str = fs::read_to_string(Path::new(path))?;
    let do_tokens = file_as_str.split("do");

    for (i, do_token) in do_tokens.enumerate() {
	if i != 0 && &do_token[0..5] == "n't()" { // don't() branch
	    continue;
	}
	else if i == 0 || &do_token[0..2] == "()" { // do() branch
	    let mult_tokens = do_token.split("mul");
	    
	    for mt in mult_tokens {
		let mut n1 = 0;
		let mut n2 = 0;
		let mut n1_was_parsed = false;
		
		if mt.len() < 5 && !mt.starts_with('(') {
		    continue;
		}
		
		for (j, c) in mt.chars().enumerate() {
		    if j == 0 && c == '(' {
			continue;
		    }
		    else if !n1_was_parsed && c.is_digit(10) && n1 < 1000 {
			n1 = n1 * 10 + c.to_digit(10).unwrap();
		    }
		    else if n1_was_parsed && c.is_digit(10) && n2 < 1000 {
			n2 = n2 * 10 + c.to_digit(10).unwrap();
		    }
		    else if c == ',' && !n1_was_parsed {
			n1_was_parsed = true;
		    }
		    else if c == ')' && n1 > 0 && n2 > 0 {
			acc += n1 * n2;
			break;
		    }
		    else {
			break;
		    }
		}
	    }
	}
    }

    println!("Part 2 => {}", acc);
    Ok(())
}

fn main() {
    let path = "input.txt";
    
    let file = File::open(path).expect("Input file not found");
    let mut reader = BufReader::new(file);
    part1(&mut reader);

    let _ = part2(path);
}
