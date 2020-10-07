pub fn run() {
    let bot_limit = 265275;
    let top_limit = 781584;

    let pass_count = (bot_limit..=top_limit)
        .into_iter()
        .map(|pass_num| pass_num.to_string().chars().collect::<Vec<char>>())
        // same adjacent exists
        .filter(|pass| {
            for (i, &char) in pass.iter().enumerate() {
                match pass.get(i + 1) {
                    Some(&val) => {
                        if char == val {
                            return true;
                        }
                    }
                    None => (),
                }
            }
            return false;
        })
        // digits are increasing
        .filter(|pass| {
            for (i, &char) in pass.iter().enumerate() {
                match pass.get(i + 1) {
                    Some(val) => {
                        if char.to_digit(10).unwrap() > val.to_digit(10).unwrap() {
                            return false;
                        }
                    }
                    None => (),
                }
            }
            return true;
        })
        .map(|pass| pass.into_iter().collect::<String>().parse::<i32>().unwrap())
        .count();

    println!("Part 1: {}", pass_count);

    let pass_count_v2 = (bot_limit..=top_limit)
        .into_iter()
        .map(|pass_num| pass_num.to_string())
        // same adjacent exists, but not in group of 3+
        .filter(|pass| {
            let get = |i: i32| {
                let j = i as usize;
                if i < 0 || j >= pass.len() {
                    return 'x';
                }

                return pass.chars().nth(j).unwrap();
            };

            for i in 0..(pass.len() - 1) {
                let i = i as i32;
                let va = get(i - 1);
                let vb = get(i);
                let vc = get(i + 1);
                let vd = get(i + 2);

                if vb == vc && va != vb && vc != vd {
                    return true;
                }
            }

            return false;
        })
        // digits are increasing
        .filter(|pass| {
            for i in 1..(pass.len()) {
                if pass[i..=i] < pass[(i - 1)..i] {
                    return false;
                }
            }
            return true;
        })
        .count();

    println!("Part 2: {}", pass_count_v2);
}
