use std::fs::read_to_string;

fn solve(measurements: &[u32]) -> u32 {
    (0..measurements.len() - 1)
        .map(|i| (measurements[i], measurements[i + 1]))
        .filter(|(x, y)| x < y)
        .fold(0, |count, _ele| count + 1)
}

fn solve2(measurements: &[u32]) -> u32 {
    // It's probably slightly more efficient not to construct the intermediate vector
    // An easy-ish way to do that would be to change solve to be able to take in an IntoIterator
    // instead, but I'm not sure how to iterate over pairs of elements.
    let sliding_measurements: Vec<u32> = (0..measurements.len() - 2)
        .map(|i| (measurements[i], measurements[i + 1], measurements[i + 2]))
        .map(|(x, y, z)| x + y + z)
        .collect();

    solve(&sliding_measurements)
}

fn main() {
    let s = read_to_string("input").expect("Failed to read input file");
    let input: Vec<u32> = s
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u32>().expect("failed to parse as integer"))
        .collect();
    println!("Solution for part 1: {}", solve(&input));
    println!("Solution for part 2: {}", solve2(&input));
}
