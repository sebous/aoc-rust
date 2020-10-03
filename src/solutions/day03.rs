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

    let mut current_pos: (i32, i32) = (0, 0);
    let mut grid: HashMap<(i32, i32), u8> = HashMap::new();

    // add first point
    grid.insert((0, 0), 1);

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
                util::range(start, target_x).for_each(|val| {
                    // println!("{:?}", (val, y));
                    if let Some(coord_value) = grid.get_mut(&(val, y)) {
                        // println!("value found {}", coord_value);
                        *coord_value += 1;
                    } else {
                        let _res = grid.insert((val, y), 1);
                    }
                });
            } else {
                let start: i32 = if y > target_y { y - 1 } else { y + 1 };
                util::range(start, target_y).for_each(|val| {
                    // println!("{:?}", (x, val));
                    if let Some(coord_value) = grid.get_mut(&(x, val)) {
                        // println!("value found {}", coord_value);
                        *coord_value += 1;
                    } else {
                        let _res = grid.insert((x, val), 1);
                    }
                })
            }

            current_pos = _target_pos;
        });
        current_pos = (0, 0);
    });
    println!("grid size: {}", grid.len());

    // TODO: doesn't work
    let intersections: Vec<(i32, i32)> = grid
        .iter()
        .filter(|(&key, &val)| val >= 2)
        .map(|(&key, val)| key)
        .collect();

    println!("itersections count: {}", intersections.len());
}
