use std::io;

fn read_input() -> Vec<String> {
	let mut line = String::new();
	let mut ret = io::stdin().read_line(&mut line);
	let mut input = Vec::new();
	loop {
		match ret {
			Ok(n) => {
				if n == 0 {
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

fn get_imm(ins: &String) -> i32 {
	let tmp: Vec<&str> = ins.split(" ").collect();
	return tmp[1].parse::<i32>().unwrap();
}

fn get_opcode(ins: &String) -> &str {
	let tmp: Vec<&str> = ins.split(" ").collect();
	return tmp[0];
}

fn execute(program: &Vec<String>) -> i32 {
	let mut flag = vec![false; program.len()];
	let mut accumulator = 0;
	let mut pc = 0;
	loop {
		if flag[pc] {
				return accumulator;
		}
		flag[pc] = true;
		match get_opcode(&program[pc]) {	
			"acc" => {
				accumulator += get_imm(&program[pc]);
				pc = pc + 1;
			}
			"nop" => pc = pc + 1,
			"jmp" => { 
				pc = (pc as i32 + get_imm(&program[pc])) as usize;
			}
			_ => ()
		}
	}
}

fn main() {
	let input: Vec<String> = read_input();
	println!("Result {}", execute(&input));
}
