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
	let mut i = 0;
	let mut j = input.len() - 1;
	loop {
		if i == j {
			break;
		}
		if input[i] + input[j] > 2020 {
			j = j - 1;
		} else if input[i] + input[j] < 2020 {
			i = i + 1;
		} else {
			println!("Result: {}", input[i] * input[j]);
			break;
		}	
	}
}
