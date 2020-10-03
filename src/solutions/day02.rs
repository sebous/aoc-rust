use crate::input;
use crate::intcode_computer::IntcodeComputer;

pub fn day02() {
    let input_program: Vec<u32> = input::load_input("inputs/02")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut input_program_p1 = Clone::clone(&input_program);
    input_program_p1[1] = 12;
    input_program_p1[2] = 2;

    let mut computer = IntcodeComputer {
        program: input_program_p1.clone(),
    };

    let part_1_memory = computer.run();
    println!("Part 1:");
    println!("{}", part_1_memory[0]);

    'outer_loop: for x in 0..99 {
        for y in 0..99 {
            let mut test_program_input = input_program.clone();
            test_program_input[1] = x;
            test_program_input[2] = y;
            computer.program = test_program_input;

            let part_2_memory = computer.run();
            if part_2_memory[0] == 19690720 {
                let noun = x;
                let verb = y;
                println!("Part 2:");
                println!("{}", 100 * noun + verb);
                break 'outer_loop;
            }
        }
    }
}
