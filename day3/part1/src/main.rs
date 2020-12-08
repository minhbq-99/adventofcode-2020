use std::io;

fn read_input() -> Vec<String> {
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	let mut input = Vec::new();
	loop {
		match ret {
			Ok(_) => {
				if line.eq(&String::from("")) {
					break;
				}
				input.push(String::from(line.trim()));
				line = String::new();
				ret = io::stdin().read_line(&mut line);
			},
			Err(_) => break,
		}
	}
	return input;
}

fn cal(input: &Vec<String>, right: usize, down: usize) -> i64 {
	let mut index = 0;
	let mut count = 0;
	for i in (down..input.len()).step_by(down) {
		let line = &input[i];
		index = (index + right) % line.len();
		if line.chars().nth(index).unwrap() == '#' {
			count = count + 1;
		}
	}
	return count;
}

fn main() {
	let input: Vec<String> = read_input();
	let ret = cal(&input, 1, 1) * cal(&input, 3, 1) * cal(&input, 5, 1) * cal(&input, 7, 1) * cal(&input, 1, 2);
	println!("{}", ret);
}
