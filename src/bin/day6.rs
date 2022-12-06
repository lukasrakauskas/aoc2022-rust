use std::collections::{HashSet, VecDeque};

fn main() -> Result<(), ()> {
    let data = include_str!("day6.input");

    println!("Problem one: {:?}", solve(data, 4));
    println!("Problem two: {:?}", solve(data, 14));

    Ok(())
}

fn solve(data: &str, size: usize) -> i32 {
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(size + 1);

    for (index, character) in data.chars().enumerate() {
        if buffer.len() == size && buffer.iter().collect::<HashSet<_>>().len() == size {
            return index as i32;
        }

        buffer.push_back(character);

        if buffer.len() > size {
            buffer.pop_front();
        }
    }

    -1
}
