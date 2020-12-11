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

fn calculate(input: &String) -> i32 {
	let mut flag = vec![true; 26];

	// trim the trailing space	
	let input = input.trim();
	let ans: Vec<&str> = input.split(" ").collect();
	for i in ans {
		let mut tmp = vec![false; 26];
		for j in i.chars() {
			tmp[j as usize - 'a' as usize] = true;
		}
		// Merge 2 flag arrays
		for j in 0..flag.len() {
			flag[j] = flag[j] && tmp[j];
		}
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
