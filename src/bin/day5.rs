use std::vec;

fn main() -> Result<(), ()> {
    println!("Problem one: {:?}", solve(handle_crane9000));
    println!("Problem two: {:?}", solve(handle_crane9001));

    Ok(())
}

type CrateStacks = Vec<Vec<char>>;
type Moves = Vec<(usize, usize, usize)>;
type Crane = fn(stacks: CrateStacks, moves: Moves) -> String;

fn solve(crane: Crane) -> String {
    let (crates_text, moves_text) = include_str!("day5.input").split_once("\r\n\r\n").unwrap();
    let stack_count = crates_text.lines().count();
    let mut stacks: CrateStacks = vec![vec![]; stack_count];

    for (i, line) in crates_text.lines().enumerate() {
        let crates = line.chars().collect::<Vec<_>>();

        let letters = crates
            .chunks(4)
            .map(|chunk| chunk.iter().filter(|item| item.is_alphabetic()).last())
            .collect::<Vec<_>>();

        if i >= stack_count - 1 {
            continue;
        }

        for (j, letter) in letters.iter().enumerate() {
            if letter.is_some() {
                stacks[j].insert(0, letter.unwrap().clone());
            }
        }
    }

    let moves: Moves = moves_text
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| !char::is_alphabetic(*c))
                .collect::<String>()
                .split(" ")
                .filter(|text| text.len() > 0)
                .map(|text| text.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|steps| steps.len() == 3)
        .map(|steps| (steps[0], steps[1], steps[2]))
        .collect::<Vec<_>>();

    crane(stacks, moves)
}

fn handle_crane9000(stacks: CrateStacks, moves: Moves) -> String {
    let mut stacks = stacks.clone();

    for (times, from, to) in moves {
        let from_stack = &mut stacks[from - 1].clone();
        let to_stack = &mut stacks[to - 1].clone();

        for _ in 0..times {
            let element = from_stack.pop();

            if element.is_some() {
                to_stack.push(element.unwrap());
            }
        }

        stacks[from - 1] = from_stack.to_vec();
        stacks[to - 1] = to_stack.to_vec();
    }

    stacks
        .iter()
        .map(|chars| chars.last())
        .filter(|it| it.is_some())
        .map(|it| *it.unwrap())
        .collect::<String>()
}

fn handle_crane9001(stacks: CrateStacks, moves: Moves) -> String {
    let mut stacks = stacks.clone();

    for (times, from, to) in moves {
        let mut from_stack = stacks[from - 1].iter().as_slice();
        let from_index = stacks[from - 1].len() - times;

        let elements = &from_stack[from_index..];
        from_stack = &from_stack[..from_index];

        let to_stack = stacks[to - 1]
            .iter()
            .chain(elements)
            .map(|c| *c)
            .collect::<Vec<_>>();

        stacks[from - 1] = from_stack.to_vec();
        stacks[to - 1] = to_stack;
    }

    stacks
        .iter()
        .map(|chars| chars.last())
        .filter(|it| it.is_some())
        .map(|it| *it.unwrap())
        .collect::<String>()
}
