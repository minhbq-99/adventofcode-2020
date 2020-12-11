 use std::io;

fn read_input() -> Vec<String> {
	let mut input: Vec<String> = Vec::new();
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	loop {
		match ret {
			Ok(n) => {
				if n == 0 {
					break;
				}
				input.push(line.trim().to_string());
				line = String::new();
				ret = io::stdin().read_line(&mut line);
			},
			Err(_) => break,
		}
	}
	return input;
}

fn binary(line: &str, num: i32, lower_char: char) -> i64 {
	let mut i = 0;
	let mut j = num;
	let mut t = 0;
	while t < line.len() {
		if line.chars().nth(t).unwrap() == lower_char {
			j = (j - i + 1) / 2 - 1 + i;
		} else {
			i = (j - i + 1) / 2 + i;
		}
		t = t + 1;
	}
	return i.into();
}

fn main() {
	let input = read_input();
	let mut flag = vec![false; 128*8];
	for i in &input {
		let m = binary(i.get(0..7).unwrap(), 127, 'F');
		let n = binary(i.get(7..10).unwrap(), 7, 'L');
		let t = m*8 + n;
		flag[t as usize] = true;
	}
	for i in 0..flag.len() {
		if flag[i] == false {
			println!("{}", i);
		}
	}
}
