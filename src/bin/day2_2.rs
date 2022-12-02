use std::collections::HashMap;

fn main() -> Result<(), ()> {
    let score_map = HashMap::from([
        ("A Z", 0),
        ("B X", 0),
        ("C Y", 0),
        ("A X", 3),
        ("B Y", 3),
        ("C Z", 3),
        ("A Y", 6),
        ("B Z", 6),
        ("C X", 6),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let replace_map = HashMap::from([
        ("X", [("A", "Z"), ("B", "X"), ("C", "Y")]),
        ("Y", [("A", "X"), ("B", "Y"), ("C", "Z")]),
        ("Z", [("A", "Y"), ("B", "Z"), ("C", "X")]),
    ]);

    let input = include_str!("day2.sample");
    let lines = input.lines();

    let mut score = 0;

    for line in lines {
        let new_line = line.replace(" ", "");
        let (them, me) = new_line.split_at(1);

        println!("{}-{}", them, me);

        let replace = replace_map.get(me);

        if replace.is_none() {
            continue;
        }

        let result = replace.unwrap().iter().find(|&(x, _me)| x == &them);

        if result.is_none() {
            continue;
        }

        let (_x, me) = result.unwrap();

        let string = format!("{them} {me}").to_string();

        let hand_score = score_map.get(me).unwrap_or(&0);
        let versus_score = score_map.get(string.as_str()).unwrap_or(&0);

        score += hand_score;
        score += versus_score;
    }

    println!("Score: {:?}", score);
    Ok(())
}
