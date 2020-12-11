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
					tmp = tmp.replace("\n", "");
					input.push(tmp);
					break;
				}
				if line == "\n" {
					tmp = tmp.replace("\n", "");
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

fn calculate(input: &String) -> i32 {
	let mut flag = vec![false; 26];
	for i in input.chars() {
		flag[i as usize - 'a' as usize] = true;
	}

	let mut count = 0;
	for i in flag {
		if i == true {
			count = count + 1;
		}
	}
	return count;
}

fn main() {
	let input = read_input();
	let mut sum = 0;
	for i in input {
		sum += calculate(&i);
	}
	println!("Result {}", sum);
}
