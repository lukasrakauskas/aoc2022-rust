use std::vec;

fn main() -> Result<(), ()> {
    let input = include_str!("day1.sample");
    let lines = input.split("\r\n");

    let mut sum = 0;
    let mut totals: Vec<i32> = vec![];

    for line in lines {
        let result = line.parse::<i32>();

        if result.is_ok() {
            sum += result.unwrap();
        } else {
            totals.push(sum);
            sum = 0;
        }
    }

    totals.push(sum);

    totals.sort_by(|a, b| b.cmp(a));

    println!("Calories: {:?}", totals[0..3].iter().sum::<i32>());
    Ok(())
}
