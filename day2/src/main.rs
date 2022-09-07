use std::fs::read_to_string;
enum Instruction {
    Forward(usize),
    Down(usize),
    Up(usize),
}

struct Position(usize, usize);

struct Position2 {
    x: i32,
    y: i32,
    aim: i32,
}

fn apply_instruction(instruction: &Instruction, pos: &mut Position) {
    match instruction {
        Instruction::Forward(d) => pos.0 += d,
        Instruction::Down(d) => pos.1 += d,
        Instruction::Up(d) => pos.1 -= d,
    }
}

fn apply_instruction2(instruction: &Instruction, pos: &mut Position2) {
    match instruction {
        Instruction::Forward(d) => {
            pos.x += *d as i32;
            pos.y += (*d as i32) * pos.aim;
        }
        Instruction::Down(d) => pos.aim += *d as i32,
        Instruction::Up(d) => pos.aim -= *d as i32,
    }
}

/* If I was really trying to do "proper" Rust, I should probably impl FromStr here,
which would then make the parse function work.  */

fn parse_instruction_exn(s: &str) -> Instruction {
    let words: Vec<&str> = s.split(' ').collect();

    if words.len() != 2 {
        panic!("Expected 2 words when parsing instruction, got {}", s)
    }

    let instruction = words[0];
    let diff = words[1]
        .parse::<usize>()
        .expect("failed to parse second word of instruction as int");

    match instruction {
        "forward" => Instruction::Forward(diff),
        "down" => Instruction::Down(diff),
        "up" => Instruction::Up(diff),
        _ => panic!("unrecognised instruction: {}", instruction),
    }
}

/* Rust would probably let me make solve be parametrised over a position type, if I made
position implement some "apply instruction" trait, and then solve and solve2 could be the same
function. */
fn solve(input: &[Instruction]) -> usize {
    let mut current_pos = Position(0, 0);

    input
        .iter()
        .for_each(|i| apply_instruction(i, &mut current_pos));

    current_pos.0 * current_pos.1
}

fn solve2(input: &[Instruction]) -> usize {
    let mut current_pos = Position2 { x: 0, y: 0, aim: 0 };

    input
        .iter()
        .for_each(|i| apply_instruction2(i, &mut current_pos));

    (current_pos.x * current_pos.y) as usize
}

fn main() {
    let s = read_to_string("input").expect("Failed to read input file");
    let input: Vec<Instruction> = s
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(parse_instruction_exn)
        .collect();

    println!("Solution for part 1: {}", solve(&input));
    println!("Solution for part 2: {}", solve2(&input));
}
