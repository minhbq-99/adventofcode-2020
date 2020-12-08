use std::io;
use regex::Regex;

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
	//let must = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];
	let mut count = 0;
	for i in &input {
		let mut have = Vec::new();
		let mut value = Vec::new();
		let token: Vec<&str> = i.split(" ").collect();
		for j in &token {
			if j == &"" {
				break;
			}
			let t: Vec<&str> = j.split(":").collect();
			have.push(t[0]);
			value.push(t[1]);
		}
		if have.contains(&"cid") {
			if !(have.len() == 8) {
				continue;
			}
		} else {
			if !(have.len() == 7) {
				continue;
			}
		}
		let mut valid = true;
		for j in 0..have.len() {
			match have[j] {
				"byr" => {
					let re = Regex::new(r"^[0-9]{4}$").unwrap();
					if !re.is_match(value[j]) {
						valid = false;
						break;
					}
					let num = value[j].parse::<i32>().unwrap();
					if num < 1920 || num > 2002 {
						valid = false;
						break;
					}
				}
				"iyr" => {
					let re = Regex::new(r"^[0-9]{4}$").unwrap();
					if !re.is_match(value[j]) {
						valid = false;
						break;
					}
					let num = value[j].parse::<i32>().unwrap();
					if num < 2010 || num > 2020 {
						valid = false;
						break;
					}
				}
				"eyr" => {
					let re = Regex::new(r"^[0-9]{4}$").unwrap();
					if !re.is_match(value[j]) {
						valid = false;
						break;
					}
					let num = value[j].parse::<i32>().unwrap();
					if num < 2020 || num > 2030 {
						valid = false;
						break;
					}
				}
				"pid" => {
					let re = Regex::new(r"^[0-9]{9}$").unwrap();
					if !re.is_match(value[j]) {
						valid = false;
						break;
					}
				}
				"ecl" => {
					let con = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
					if !(con.contains(&value[j])) {
						valid = false;
						break;
					}
				}
				"hcl" => {
					let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
					if !re.is_match(value[j]) {
						valid = false;
						break;
					}
				}
				"hgt" => {
					let len = value[j].len();
					//println!("{}", value[j]);
					let ret = value[j].get(0..(len-2)).unwrap().parse::<i32>();
					let num;
					match ret {
						Ok(n) => num = n,
						Err(_) => {
							valid = false;
							break;
						}
					}
					if value[j].get((len - 2)..len).unwrap() == "cm" {
						if num < 150 || num > 193 {
							valid = false;
							break;
						}
					} else if value[j].get((len - 2)..len).unwrap() == "in" {
						if num < 59 || num > 76 {
							valid = false;
							break;
						}
					} else {
						valid = false;
						break;
					}
				}
				_ => (),
			}
		}
		if valid {
			count = count + 1;
		}
	}
	println!("Result {}", count);
}
