const INPUT: &str = include_str!("../../../inputs/problem1_input");
pub fn day1_part1_solution(){
    let input: Vec<i32> = INPUT
        .trim()
        .split("\n")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();
    let mut times_incleased = 0;
    let mut prev_num = &input[0];
    for nr in input.iter().skip(1) {
        if nr > prev_num {
            times_incleased += 1;
        }
        prev_num = nr;
    }
    println!("PART 1 solution: {}", times_incleased);
}

pub fn day1_part2_solution(){
    let window_width = 3;
    let input: Vec<i32> = INPUT
        .trim()
        .split("\n")
        .filter_map(|n| n.parse::<i32>().ok())
        .collect();

    let mut times_incleased = 0;
    let mut prev_sum = 0;
    for i in 0..window_width {
        prev_sum += input[i];
    }
    for i in window_width..input.len() {
        let sum = prev_sum + input[i] - input[i - window_width];
        if sum > prev_sum {
            times_incleased += 1;
        }
        prev_sum = sum;
    }
    println!("PART 2 solution: {}", times_incleased);
}

/*
 * https://github.com/timvisee/advent-of-code-2021/blob/master/day01b/src/main.rs
 *
 *     println!(
        "{}",
        include_str!("../input.txt")
            .lines()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u16>>()
            .array_windows()
            .filter(|[a, _, _, d]| a < d)
            .count(),
    );
*/
