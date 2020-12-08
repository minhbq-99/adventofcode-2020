use std::io;

fn main() {
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	let mut input = Vec::new();
	loop {
		match ret {
			Ok(_) => {
				if line.eq(&String::from("")) {
					break;
				}
				input.push(line.trim().parse::<i32>().unwrap());
				line = String::new();
				ret = io::stdin().read_line(&mut line);
			},
			Err(_) => break,
		}
	}
	input.sort();
	for i in 0..input.len() {
		let mut j = i + 1;
		let mut t = input.len() - 1;
		loop {
			if j == t {
				break;
			}
			if input[j] + input[t] + input[i] > 2020 {
				t = t - 1;
			} else if input[j] + input[t] + input[i] < 2020 {
				j = j + 1;
			} else {
				println!("Result {}", input[i]*input[j]*input[t]);
				return;
			}
		}
	}		
}
