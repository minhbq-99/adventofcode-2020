use std::io;
use std::env;

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

fn cal(input: &Vec<i64>, i: usize, j: usize) -> i64 {
	let mut min = 10000000000000;
	let mut max = 0;
	for t in i..=j {
		if input[t] < min {
			min = input[t];
		}
		if input[t] > max {
			max = input[t];
		}
	}
	return min + max;
}

fn main() {
	let input = read_input();
	let mut t: i64 = -1;
	let num = env::args().nth(1).unwrap().parse::<usize>().unwrap();
	for i in num..(input.len()) {
		let mut found = false;
		for j in (i-num)..i {
			for k in (j+1)..i {
				if input[j] + input[k] == input[i] {
					found = true;
					break;
				}
			}
		}
		if !found {
			t = input[i];
			break;
		}
	}
	
	let mut i: usize = 0;
	let mut j: usize = 0;
	let mut sum = input[i];
	loop {
		if sum == t {
			println!("Result {}", cal(&input, i, j));
			return;
		}
		j = j + 1;
		sum = sum + input[j];
		while sum > t {
			sum = sum - input[i];
			i = i + 1;
		}
	}
}
