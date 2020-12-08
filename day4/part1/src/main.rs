use std::io;

fn read_input() -> Vec<String> {
	let mut input: Vec<String> = Vec::new();
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	let mut tmp = String::new();
	loop {
		match ret {
			Ok(n) => {
				if n == 0 {
					tmp = tmp.replace("\n", " ");
					input.push(tmp);
					break;
				}
				if line == "\n" {
					tmp = tmp.replace("\n", " ");
					input.push(tmp);
					tmp = String::new();
					line.clear();
				}
				
				tmp = tmp + &line;
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
	let must = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
	let mut count = 0;
	for i in &input {
		let mut have = Vec::new();
		let token: Vec<&str> = i.split(" ").collect();
		for j in &token {
			let t: Vec<&str> = j.split(":").collect();
			have.push(t[0]);
		}
		let mut valid = true;
		for j in &must {
			if !have.contains(&j) {
				valid = false;
				break;
			}
		}
		if valid {
			count = count + 1;
		}
	}
	println!("Result {}", count);
}
