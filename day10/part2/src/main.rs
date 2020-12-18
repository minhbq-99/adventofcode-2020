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
	let mut num = 1;
	let mut i = 0;
	while i < input.len() - 1 {
		let mut j = i + 1;
		while (j < input.len()) && (input[j] - input[i] <= 3) {
			j = j + 1;
		}
		i = j - 1;
		num = num + 1;
	}
	let ret: i64 = 2 << ((input.len() - num) as i64);
	println!("Result {}", ret); 
}
