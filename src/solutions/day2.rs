const INPUT: &str = include_str!("../../../inputs/day2_input");

pub fn day2_part1_solution() {
    let mut x = 0;
    let mut y = 0;
    let instructions: Vec<&str> = INPUT
        .lines()
        .flat_map(|line| line.split(" "))
        .collect::<Vec<&str>>();

    for instruction in instructions.chunks(2) {
        let movement = instruction[0];
        let amount = instruction[1].parse::<u32>().unwrap();
        match (movement, amount) {
            ("forward", _) => x += amount,
            ("up", _) => y -= amount,
            ("down", _) => y += amount,
            (_, _) => println!("Cound not match {} {}", movement, amount),
        }
    }
    println!("PART 1 solution {}", x * y);

}

pub fn day2_part2_solution() {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    let instructions: Vec<&str> = INPUT
        .lines()
        .flat_map(|line| line.split(" "))
        .collect::<Vec<&str>>();

    for instruction in instructions.chunks(2) {
        let movement = instruction[0];
        let amount = instruction[1].parse::<u32>().unwrap();
        match (movement, amount) {
            ("forward", _) => {
                x += amount;
                y += aim * amount;
            },
            ("up", _) => aim -= amount,
            ("down", _) => aim += amount,
            (_, _) => println!("Cound not match {} {}", movement, amount),
        }
    }
    println!("PART 2 solution {}", x * y);
}
