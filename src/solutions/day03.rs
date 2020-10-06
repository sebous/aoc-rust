use crate::{input, util};
use std::collections::HashMap;

pub fn run() {
    let program_one: Vec<String> = input::load_input("inputs/03")
        .lines()
        .take(1)
        .collect::<String>()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let program_two: Vec<String> = input::load_input("inputs/03")
        .lines()
        .skip(1)
        .take(1)
        .collect::<String>()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    struct Intersection {
        pub x: u8,
        pub y: u8,
    }

    let mut current_pos: (i32, i32) = (0, 0);
    let mut grid: HashMap<(i32, i32), (u8, u8)> = HashMap::new();

    // add first point
    grid.insert((0, 0), (0, 0));

    [program_one, program_two].iter().for_each(|program| {
        program.iter().for_each(|instr| {
            let distance = &instr[1..].parse::<i32>().unwrap();
            let (x, y) = current_pos;
            let mut _target_pos: (i32, i32) = (0, 0);

            // first char
            match instr.chars().next().unwrap() {
                'R' => _target_pos = (x + distance, y),
                'L' => _target_pos = (x - distance, y),
                'U' => _target_pos = (x, y + distance),
                'D' => _target_pos = (x, y - distance),
                _ => panic!("Wrong instruction!"),
            }
            let (target_x, target_y) = _target_pos;

            // println!("instr: {}", instr);
            // println!("target pos: {:?}", _target_pos);
            if x != target_x {
                let start: i32 = if x > target_x { x - 1 } else { x + 1 };
                // println!(
                //     "{:?}",
                //     util::range(start, target_x)
                //         .into_iter()
                //         .collect::<Vec<i32>>(),
                // );
                util::range(start, target_x).for_each(|val| {
                    // println!("{:?}", (val, y));
                    if let Some(coord_value) = grid.get_mut(&(val, y)) {
                        *coord_value = (1, coord_value.1);
                    } else {
                        let _res = grid.insert((val, y), (1, 0));
                    }
                });
            } else {
                let start: i32 = if y > target_y { y - 1 } else { y + 1 };
                // println!(
                //     "{:?}",
                //     util::range(start, target_y)
                //         .into_iter()
                //         .collect::<Vec<i32>>(),
                // );
                util::range(start, target_y).for_each(|val| {
                    // println!("{:?}", (x, val));
                    if let Some(coord_value) = grid.get_mut(&(x, val)) {
                        *coord_value = (coord_value.0, 1);
                    } else {
                        let _res = grid.insert((x, val), (0, 1));
                    }
                })
            }

            current_pos = _target_pos;
        });
        current_pos = (0, 0);
    });
    println!("grid size: {}", grid.len());

    let intersections: Vec<(i32, i32)> = grid
        .iter()
        .filter(|(&_key, &coord_value)| coord_value.0 == 1 && coord_value.1 == 1)
        .map(|(&key, _val)| key)
        .collect();

    println!("itersections count: {}", intersections.len());

    let minimal_distance = intersections
        .iter()
        .map(|&pos| {
            // println!("{:#?}", pos);
            return pos.0.abs() + pos.1.abs();
        })
        .min()
        .unwrap();

    println!("Part 1: {}", minimal_distance);
}
