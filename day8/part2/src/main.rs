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

fn execute(program: &Vec<String>) -> (Vec<usize>, i32) {
	let mut flag = vec![false; program.len()];
	let mut accumulator = 0;
	let mut pc = 0;
	let mut ret = Vec::new();
	loop {
		if pc == program.len() {
			return (Vec::new(), accumulator);
		}
		if flag[pc] {
				return (ret, 0);
		}
		flag[pc] = true;
		match get_opcode(&program[pc]) {	
			"acc" => {
				accumulator += get_imm(&program[pc]);
				pc = pc + 1;
			}
			"nop" => pc = pc + 1,
			"jmp" => {
				ret.push(pc);
				pc = (pc as i32 + get_imm(&program[pc])) as usize;
			}
			_ => ()
		}
	}
}

fn main() {
	let input: Vec<String> = read_input();

	let jmp_ins = execute(&input).0;
	//println!("{}", jmp_ins.len());
	for i in (0..jmp_ins.len()).rev() {
		let j = jmp_ins[i];
		//println!("{}", j);
		let mut tmp = input.clone();
		tmp[j] = String::from("nop");
		let ret = execute(&tmp);
		if ret.0.len() == 0 {
			println!("Result {}", ret.1);
			return;
		}
	}
}
