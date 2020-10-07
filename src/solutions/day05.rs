use crate::{input, intcode_computer};

pub fn run() {
    let program: Vec<i32> = input::load_input("inputs/05")
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut computer = intcode_computer::IntcodeComputer::new(program);
    computer.run();
}
