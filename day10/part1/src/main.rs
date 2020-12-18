use std::io;

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
	let mut input = read_input();
	input.push(0);
	input.sort();
	let mut num_1 = 0;
	let mut num_3 = 1;
	for i in 1..input.len() {
		let t = input[i] - input[i-1];
		if t == 1 {
			num_1 = num_1 + 1;
		} else if t == 3 {
			num_3 = num_3 + 1;
		}
	}
	println!("Result {}", num_1*num_3);
}
