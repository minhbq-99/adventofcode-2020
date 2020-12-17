use std::io;
use std::env;

fn read_input() -> Vec<i64> {
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	let mut input = Vec::new();
	loop {
		match ret {
			Ok(n) => {
				if n == 0 {
					break;
				}
				input.push(line.trim().parse::<i64>().unwrap());
				line = String::new();
				ret = io::stdin().read_line(&mut line);
			},
			Err(_) => break,
		}
	}
	return input;
}

fn main() {
	let input = read_input();
	let num = env::args().nth(1).unwrap().parse::<usize>().unwrap();
	for i in num..(input.len()) {
		let mut found = false;
		for j in (i-num)..i {
			for k in (j+1)..i {
				if input[j] + input[k] == input[i] {
					found = true;
					break;
				}
			}
		}
		if !found {
			println!("Result {}", input[i]);
			return;
		}
	}
}
