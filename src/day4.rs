// 130254-678275

#[must_use]
fn validate_password(candidate: usize, part: usize) -> bool {
    let candidate: Vec<_> = format!("{}", candidate).chars().collect();
    let mut sorted_clone = candidate.clone();
    sorted_clone.sort_unstable();

    let hmap = sorted_clone
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, digit| {
            *acc.entry(digit).or_default() += 1_usize;
            acc
        });
    let repeated = if part == 2 {
        hmap.values().any(|v: &usize| v == &2)
    } else {
        hmap.values().any(|v: &usize| v >= &2)
    };

    // Growing order validation
    let growing = candidate[0] <= candidate[5] && sorted_clone == candidate;

    repeated && growing
}

fn main() {
    let min = 130_254;
    let max = 678_275;

    let count = (min..=max)
        .filter(|&candidate| validate_password(candidate, 1))
        .count();
    println!("Part 1: {}", count);

    let count = (min..=max)
        .filter(|&candidate| validate_password(candidate, 2))
        .count();
    println!("Part 2: {}", count);
}
