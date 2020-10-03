pub struct IntcodeComputer {
    pub program: Vec<u32>,
}
struct InstrParams {
    pub first_param: usize,
    pub second_param: usize,
    pub third_param: usize,
}

impl IntcodeComputer {
    pub fn run(&self) -> Vec<u32> {
        const INSTR_SIZE: usize = 4;
        const EXIT_OP: u32 = 99;
        let mut memory = self.program.clone();

        let mut memory_pointer: usize = 0;
        let mut op_code: u32;

        loop {
            op_code = memory[memory_pointer];
            if op_code == EXIT_OP {
                break;
            }
            let params = InstrParams {
                first_param: memory[memory_pointer + 1] as usize,
                second_param: memory[memory_pointer + 2] as usize,
                third_param: memory[memory_pointer + 3] as usize,
            };

            // println!("--------");
            // println!("{}", op_code);
            // println!("{:?}", memory);
            // println!("{}", memory[params.third]);
            // println!("--------");
            match op_code {
                1 => {
                    memory[params.third_param] =
                        memory[params.first_param] + memory[params.second_param]
                }
                2 => {
                    memory[params.third_param] =
                        memory[params.first_param] * memory[params.second_param]
                }
                _ => panic!("Unexpected op_code: {}", op_code),
            }

            memory_pointer += INSTR_SIZE;
        }
        // println!("{:?}", memory);
        return memory;
    }
}
