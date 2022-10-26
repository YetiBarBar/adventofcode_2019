fn main() {
    let input = include_str!("../data/day_2019_1.data");
    let part1 = compute(input, 1);
    let part2 = compute(input, 2);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn compute(input: &str, step: usize) -> isize {
    let value = input
        .lines()
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .map(|l| l / 3 - 2)
        .map(|l| {
            let mut additional: isize = l;
            let mut to_add: isize = 0;
            if step == 2 {
                while additional > 0 {
                    if (additional / 3 - 2) > 0 {
                        to_add += additional / 3 - 2;
                    }
                    additional = additional / 3 - 2;
                }
            }
            l + to_add
        })
        .sum::<isize>();
    value
}
