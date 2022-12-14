use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug)]
enum Cmd {
    Left(isize),
    Right(isize),
    Up(isize),
    Down(isize),
}

impl FromStr for Cmd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse::<isize>().unwrap();
        match s.chars().next() {
            Some('L') => Ok(Self::Left(value)),
            Some('R') => Ok(Self::Right(value)),
            Some('D') => Ok(Self::Down(value)),
            Some('U') => Ok(Self::Up(value)),
            _ => Err(()),
        }
    }
}

struct Line {
    timed: HashMap<usize, (isize, isize)>,
    points: HashSet<(isize, isize)>,
}

impl From<&Vec<Cmd>> for Line {
    fn from(point: &Vec<Cmd>) -> Self {
        let timed = point
            .iter()
            .fold(
                (
                    HashMap::<usize, (isize, isize)>::new(),
                    (0_isize, 0_isize),
                    0_usize,
                ),
                |(mut acc, (x, y), mut count), cmd| {
                    let (x, y) = match cmd {
                        Cmd::Left(v) => {
                            for idx in 1..=*v {
                                count += 1;
                                acc.insert(count, (x - idx, y));
                            }
                            (x - v, y)
                        }
                        Cmd::Right(v) => {
                            for idx in 1..=*v {
                                count += 1;
                                acc.insert(count, (x + idx, y));
                            }
                            (x + v, y)
                        }
                        Cmd::Up(v) => {
                            for idx in 1..=*v {
                                count += 1;
                                acc.insert(count, (x, y + idx));
                            }
                            (x, y + v)
                        }
                        Cmd::Down(v) => {
                            for idx in 1..=*v {
                                count += 1;
                                acc.insert(count, (x, y - idx));
                            }
                            (x, y - v)
                        }
                    };
                    (acc, (x, y), count)
                },
            )
            .0;
        Self {
            points: timed.values().copied().collect(),
            timed,
        }
    }
}

fn common_point(line_a: &Line, line_b: &Line) -> Vec<(isize, isize)> {
    line_a
        .points
        .iter()
        .filter(|point| line_b.points.contains(point))
        .copied()
        .collect()
}

fn main() {
    let input: Vec<Vec<Cmd>> = include_str!("../data/day_2019_3.data")
        .lines()
        .map(|line| {
            line.split(',')
                .map(str::parse::<Cmd>)
                .map(Result::unwrap)
                .collect()
        })
        .collect();
    let line0 = Line::from(&input[0]);
    let line1 = Line::from(&input[1]);

    let cp = common_point(&line0, &line1);

    // println!("Common points: {:?}", cp);

    let min = cp.iter().map(|(x, y)| x.abs() + y.abs()).min().unwrap();
    println!("Part 1: {}", min);

    let min2 = cp
        .iter()
        .map(|point| {
            line0.timed.iter().find(|&(_, val)| val == point).unwrap().0
                + line1.timed.iter().find(|&(_, val)| val == point).unwrap().0
        })
        .min()
        .unwrap();

    println!("Part 2: {}", min2);
}
