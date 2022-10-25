use adventofcode_2019::intcode::Cpu;

fn main() {
    let prog: Vec<_> = include_str!("../data/day_2019_2.data")
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let mut cpu = Cpu::new(&prog);
    cpu.reset_to_value(12, 2);

    println!("Part 1: {}", cpu.run_all());

    let part2 = (0_usize..100)
        .flat_map(move |noun: usize| (0_usize..100).map(move |verb| (noun, verb)))
        .find(|(noun, verb)| {
            cpu.reset_to_value(*noun, *verb);
            cpu.run_all() == 19_690_720
        });

    if let Some((noun, verb)) = part2 {
        println!("Part 2: {:2}{:2}", noun, verb);
    }
}
