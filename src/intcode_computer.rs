use std::io::{self, Read};

pub struct IntcodeComputer {
    memory: Vec<i32>,
    pointer: usize,
}

enum ParamMode {
    POSITION,
    IMMEDIATE,
}

impl IntcodeComputer {
    pub fn new(program: Vec<i32>) -> Self {
        IntcodeComputer {
            memory: program.clone(),
            pointer: 0,
        }
    }

    pub fn run(&mut self) -> Vec<i32> {
        let instruction = |instr: i32| -> ((ParamMode, ParamMode, ParamMode), u32) {
            let instr = format!("{:05}", instr);
            let len = instr.len();

            let param_modes: Vec<ParamMode> = instr
                .chars()
                .take(3)
                .map(|ch| match ch {
                    '0' => return ParamMode::POSITION,
                    '1' => return ParamMode::IMMEDIATE,
                    _ => panic!("wrong paramterer mode!"),
                })
                .collect();
            let param_modes_tuple = (param_modes[0], param_modes[1], param_modes[2]);
            let op_code: u32 = instr[3..4].parse().unwrap();

            return (param_modes_tuple, op_code);
        };
        const EXIT_OP: u32 = 99;

        let get_param = |param_i: usize, param_mode: ParamMode| -> i32 {
            match param_mode {
                ParamMode::POSITION => {
                    let i = self.memory[self.pointer + param_i + 1];
                    return self.memory[i as usize];
                }
                ParamMode::IMMEDIATE => self.memory[self.pointer + param_i + 1],
                _ => panic!("wrong parameter mode"),
            }
        };

        loop {
            let instruction = instruction(self.memory[self.pointer]);
            let (param_modes, op_code) = instruction;
            if op_code == EXIT_OP {
                break;
            }

            match op_code {
                1 => {
                    &self.memory[self.memory[self.pointer + 3] as usize] =
                        get_param(0, param_modes.0) + get_param(1, param_modes.1);
                    self.pointer += 4;
                }
                2 => {
                    self.memory[self.memory[self.pointer + 3] as usize] =
                        get_param(0, param_modes.0) * get_param(1, param_modes.1);
                    self.pointer += 4;
                }
                3 => {
                    let mut buffer = String::new();
                    io::stdin().read_to_string(&mut buffer).unwrap();
                    self.memory[self.memory[self.pointer + 1] as usize] =
                        buffer.parse::<i32>().unwrap();

                    self.pointer += 2;
                }
                4 => {
                    println!("{}", get_param(0, param_modes.0));
                    self.pointer += 2;
                }
                _ => panic!("Unexpected op_code: {}", op_code),
            }
        }
        return self.memory;
    }
}
