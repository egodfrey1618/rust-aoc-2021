use std::fs::read_to_string;

fn number_of_bits(input: &[usize]) -> usize {
    let mut max = input.iter().fold(0, |acc, &t| usize::max(acc, t));

    let mut number_of_bits = 0;
    while max > 0 {
        max /= 2;
        number_of_bits += 1;
    }
    number_of_bits
}

fn get_bit(x: usize, bit: usize) -> usize {
    let mask = 1 << bit;

    (x & mask) >> bit
}

enum BitPattern {
    AllOnes,
    AllZeros,
    Equal,
    MoreZerosThanOnes,
    MoreOnesThanZeros,
}

fn bit_pattern(input: &[usize], bit: usize) -> BitPattern {
    let (number_of_zeros, number_of_ones) =
        input
            .iter()
            .fold((0, 0), |(count0, count1), &this_integer| {
                if get_bit(this_integer, bit) == 1 {
                    (count0, count1 + 1)
                } else {
                    (count0 + 1, count1)
                }
            });

    match (number_of_zeros, number_of_ones) {
        (0, _) => BitPattern::AllOnes,
        (_, 0) => BitPattern::AllZeros,
        _ => match number_of_zeros.cmp(&number_of_ones) {
            std::cmp::Ordering::Less => BitPattern::MoreOnesThanZeros,
            std::cmp::Ordering::Greater => BitPattern::MoreZerosThanOnes,
            std::cmp::Ordering::Equal => BitPattern::Equal,
        },
    }
}

fn gamma_rate(input: &[usize]) -> usize {
    let number_of_bits = number_of_bits(input);

    (0..number_of_bits)
        .map(|bit| {
            let x = match bit_pattern(input, bit) {
                BitPattern::AllOnes => 1,
                BitPattern::MoreOnesThanZeros => 1,
                BitPattern::AllZeros => 0,
                BitPattern::MoreZerosThanOnes => 0,
                BitPattern::Equal => panic!("don't know what to do if # of bits equal"),
            };
            (bit, x)
        })
        .fold(0, |acc, (bit, x)| acc + (x * (1 << bit)))
}

fn epsilon_rate_of_gamma_rate(gamma_rate: usize, number_of_bits: usize) -> usize {
    // We could also compute this similarly to gamma_rate, but passing in least_common_bit instead
    let mask = (1 << number_of_bits) - 1;
    gamma_rate ^ mask
}

fn oxygen_generator_rating(input: &[usize]) -> usize {
    let number_of_bits = number_of_bits(input);
    let input_as_vec = input.to_vec();
    let surviving_values = (0..number_of_bits)
        .rev()
        .fold(input_as_vec, |values, bit| match values.len() {
            1 => values,
            0 => panic!("filter_readings_until_unique got empty list"),
            _ => {
                let keep_bit = match bit_pattern(&values, bit) {
                    BitPattern::AllOnes => 1,
                    BitPattern::MoreOnesThanZeros => 1,
                    BitPattern::AllZeros => 0,
                    BitPattern::MoreZerosThanOnes => 0,
                    BitPattern::Equal => 1,
                };
                values
                    .iter()
                    .filter(|x| get_bit(**x, bit) == keep_bit)
                    .copied()
                    .collect::<Vec<usize>>()
            }
        });

    surviving_values[0]
}

// There's a lot of code duplication between this and the oxygen generator. I struggled to remove this with a closure.
// The only difference is in what we take for the BitPattern.
fn co2_scrubber_rating(input: &[usize]) -> usize {
    let number_of_bits = number_of_bits(input);
    let input_as_vec = input.to_vec();
    let surviving_values = (0..number_of_bits)
        .rev()
        .fold(input_as_vec, |values, bit| match values.len() {
            1 => values,
            0 => panic!("filter_readings_until_unique got empty list"),
            _ => {
                let keep_bit = match bit_pattern(&values, bit) {
                    // This is not quite the opposite of the pattern in oxygen scrubber.
                    // If they're all the same, we want to keep that one, rather than chuck everything out.
                    BitPattern::AllOnes => 1,
                    BitPattern::MoreOnesThanZeros => 0,
                    BitPattern::AllZeros => 0,
                    BitPattern::MoreZerosThanOnes => 1,
                    BitPattern::Equal => 0,
                };
                values
                    .iter()
                    .filter(|x| get_bit(**x, bit) == keep_bit)
                    .copied()
                    .collect::<Vec<usize>>()
            }
        });

    surviving_values[0]
}

fn solve(input: &[usize]) -> usize {
    let gamma_rate = gamma_rate(input);
    let number_of_bits = number_of_bits(input);
    let epsilon_rate = epsilon_rate_of_gamma_rate(gamma_rate, number_of_bits);
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Number of bits: {}", number_of_bits);
    gamma_rate * epsilon_rate
}
fn solve2(input: &[usize]) -> usize {
    println!("oxygen: {}", oxygen_generator_rating(input));
    println!("CO2: {}", co2_scrubber_rating(input));

    oxygen_generator_rating(input) * co2_scrubber_rating(input)
}
fn main() {
    let s = read_to_string("input").expect("Failed to read input file");
    let input: Vec<usize> = s
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| usize::from_str_radix(s, 2).expect("unable to convert to integer"))
        .collect();

    println!("Solution for part 1: {}", solve(&input));
    println!("Solution for part 2: {}", solve2(&input));
}
