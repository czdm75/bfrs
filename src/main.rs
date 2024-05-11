use std::io::Read;

fn main() {
    let program_str = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let quine = "->+>+++>>+>++>+>+++>>+>++>>>+>+>+>++>+>>>>+++>+>>++>+>+++>>++>++>>+>>+>++>++>+>>>>+++>+>>>>++>++>>>>+>>++>+>+++>>>++>>++++++>>+>>++>+>>>>+++>>+++++>>+>+++>>>++>>++>>+>>++>+>+++>>>++>>+++++++++++++>>+>>++>+>+++>+>+++>>>++>>++++>>+>>++>+>>>>+++>>+++++>>>>++>>>>+>+>++>>+++>+>>>>+++>+>>>>+++>+>>>>+++>>++>++>+>+++>+>++>++>>>>>>++>+>+++>>>>>+++>>>++>+>+++>+>+>++>>>>>>++>>>+>>>++>+>>>>+++>+>>>+>>++>+>++++++++++++++++++>>>>+>+>>>+>>++>+>+++>>>++>>++++++++>>+>>++>+>>>>+++>>++++++>>>+>++>>+++>+>+>++>+>+++>>>>>+++>>>+>+>>++>+>+++>>>++>>++++++++>>+>>++>+>>>>+++>>++++>>+>+++>>>>>>++>+>+++>>+>++>>>>+>+>++>+>>>>+++>>+++>>>+[[->>+<<]<+]+++++[->+++++++++<]>.[+]>>[<<+++++++[->+++++++++<]>-.------------------->-[-<.<+>>]<[+]<+>>>]<<<[-[-[-[>>+<++++++[->+++++<]]>++++++++++++++<]>+++<]++++++[->+++++++<]>+<<<-[->>>++<<<]>[->>.<<]<<]";
    let capitalize = ",----------[----------------------.,----------]";
    run_program(program_str);
    run_program(quine);
    println!();
    run_program_immut(program_str);
    run_program_immut(quine);
    println!();
    run_program(capitalize);
}

fn run_program(program_str: &str) {
    let program: Vec<char> = program_str.chars().collect();
    let instructions = (0..program.len())
        .map(|idx| Instruction::from(idx, &program))
        .collect();
    let state = &mut State::new();
    while state.interpret(&instructions) {}
}

fn run_program_immut(program_str: &str) {
    let program: Vec<char> = program_str.chars().collect();
    let instructions = (0..program.len())
        .map(|idx| Instruction::from(idx, &program))
        .collect();
    let mut state = State::new();
    loop {
        let state_opt = state.interpret_immut(&instructions);
        match state_opt {
            Some(s) => {
                state = s
            }
            None => {
                break;
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Input,
    Output,
    Inc,
    Dec,
    JmpForward(usize),
    JmpBack(usize),
}

fn search_right_bracket(pos: usize, vec: &Vec<char>) -> Option<usize> {
    let mut bracket_cnt = 0;
    let mut p = pos + 1;
    while p < vec.len() {
        if bracket_cnt == 0 && vec[p] == ']' {
            return Some(p);
        } else if vec[p] == '[' {
            bracket_cnt += 1;
        } else if vec[p] == ']' {
            bracket_cnt -= 1;
        }
        p += 1;
    }
    None
}

fn search_left_bracket(pos: usize, vec: &[char]) -> Option<usize> {
    let mut bracket_cnt = 0;
    let mut p = pos - 1;
    while p > 0 {
        if bracket_cnt == 0 && vec[p] == '[' {
            return Some(p);
        } else if vec[p] == ']' {
            bracket_cnt += 1;
        } else if vec[p] == '[' {
            bracket_cnt -= 1;
        }
        p -= 1;
    }
    None
}

impl Instruction {
    fn from(pc: usize, program: &Vec<char>) -> Self {
        match program[pc] {
            '+' => Instruction::Inc,
            '-' => Instruction::Dec,
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            ',' => Instruction::Input,
            '.' => Instruction::Output,
            '[' => Instruction::JmpForward(search_right_bracket(pc, program).unwrap()),
            ']' => Instruction::JmpBack(search_left_bracket(pc, program).unwrap()),
            _ => panic!("unknown instruction"),
        }
    }
}

#[derive(Debug)]
struct State {
    pc: usize,
    ptr: usize,
    mem: Vec<u8>,
}

impl State {
    fn new() -> State {
        State {
            pc: 0,
            ptr: 0,
            mem: Vec::from([0]),
        }
    }

    fn interpret_immut(self, insts: &Vec<Instruction>) -> Option<State> {
        if self.pc == insts.len() {
            None
        } else {
            let mut pc = self.pc;
            let mut ptr = self.ptr;
            let mut mem = self.mem;
            match insts[self.pc] {
                Instruction::Left => {
                    ptr -= 1;
                }
                Instruction::Right => {
                    ptr += 1;
                    if ptr == mem.len() {
                        mem.push(0)
                    }
                }
                Instruction::Input => {
                    let input = std::io::stdin().bytes().next().unwrap().unwrap();
                    mem[ptr] = input;
                }
                Instruction::Output => {
                    print!("{}", char::from(mem[ptr]));
                }
                Instruction::Inc => {
                    if mem[ptr] == u8::MAX {
                        mem[ptr] = 0;
                    } else {
                        mem[ptr] += 1;
                    }
                }
                Instruction::Dec => {
                    if mem[ptr] == 0 {
                        mem[ptr] = u8::MAX;
                    } else {
                        mem[ptr] -= 1;
                    }
                }
                Instruction::JmpForward(tgt) => {
                    if mem[ptr] == 0 {
                        pc = tgt;
                    }
                }
                Instruction::JmpBack(tgt) => {
                    if mem[ptr] != 0 {
                        pc = tgt;
                    }
                }
            }
            pc += 1;
            Some(State { pc, ptr, mem })
        }
    }

    fn interpret(self: &mut State, insts: &Vec<Instruction>) -> bool {
        if self.pc == insts.len() {
            false
        } else {
            match insts[self.pc] {
                Instruction::Left => {
                    self.ptr -= 1;
                }
                Instruction::Right => {
                    self.ptr += 1;
                    if self.ptr == self.mem.len() {
                        self.mem.push(0)
                    }
                }
                Instruction::Input => {
                    let input = std::io::stdin().bytes().next().unwrap().unwrap();
                    self.mem[self.ptr] = input;
                }
                Instruction::Output => {
                    print!("{}", char::from(self.mem[self.ptr]));
                }
                Instruction::Inc => {
                    if self.mem[self.ptr] == u8::MAX {
                        self.mem[self.ptr] = 0;
                    } else {
                        self.mem[self.ptr] += 1;
                    }
                }
                Instruction::Dec => {
                    if self.mem[self.ptr] == 0 {
                        self.mem[self.ptr] = u8::MAX;
                    } else {
                        self.mem[self.ptr] -= 1;
                    }
                }
                Instruction::JmpForward(tgt) => {
                    if self.mem[self.ptr] == 0 {
                        self.pc = tgt;
                    }
                }
                Instruction::JmpBack(tgt) => {
                    if self.mem[self.ptr] != 0 {
                        self.pc = tgt;
                    }
                }
            }
            self.pc += 1;
            true
        }
    }
}
