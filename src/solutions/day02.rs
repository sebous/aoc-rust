use crate::input;

struct InstrParams {
    pub first_param: usize,
    pub second_param: usize,
    pub third_param: usize,
}

fn intcode_computer(program: Vec<u32>) -> Vec<u32> {
    const INSTR_SIZE: usize = 4;
    const EXIT_OP: u32 = 99;
    let mut memory = Vec::from(program);

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

pub fn day02() {
    let input_program: Vec<u32> = input::load_input("inputs/02")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut input_program_p1 = Clone::clone(&input_program);
    input_program_p1[1] = 12;
    input_program_p1[2] = 2;

    let part_1_memory = intcode_computer(input_program_p1);
    println!("Part 1:");
    println!("{}", part_1_memory[0]);

    let mut noun: u32;
    let mut verb: u32;

    'outer_loop: for x in 0..99 {
        for y in 0..99 {
            let mut test_program_input = Clone::clone(&input_program);
            test_program_input[1] = x;
            test_program_input[2] = y;

            let part_2_memory = intcode_computer(test_program_input);
            if part_2_memory[0] == 19690720 {
                noun = x;
                verb = y;
                println!("{}", 100 * noun + verb);
                break 'outer_loop;
            }
        }
    }
}
