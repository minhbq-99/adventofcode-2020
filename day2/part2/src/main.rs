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

fn is_valid(line: &String) -> bool {
	let split: Vec<&str> = line.split(" ").collect();

	let first: Vec<&str> = split[0].split("-").collect();
	let min = first[0].parse::<usize>().unwrap() - 1;
	let max = first[1].parse::<usize>().unwrap() - 1;

	let chr = split[1].chars().nth(0).unwrap();
	let password = split[2];

	let mut count = 0;
	
	if min < password.len() && password.chars().nth(min).unwrap() == chr {
		count = count + 1;
	}

	if max < password.len() && password.chars().nth(max).unwrap() == chr {
		count = count + 1;
	}

	if count != 1 {
		return false;
	}

	return true;
}

fn main() {
	let input: Vec<String> = read_input();
	let mut num_valid = 0;
	for line in &input {
		if is_valid(&line) {
			num_valid = num_valid + 1;
		}
	}
	println!("Result {}", num_valid);
}
