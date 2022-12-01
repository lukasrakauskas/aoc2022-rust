fn main() {
    let input = include_str!("./sample.txt");
    let lines = input.split("\r\n");

    let mut max = 0;
    let mut sum = 0;

    for line in lines {
        let result = line.parse::<i32>();

        if result.is_ok() {
            let calories = result.unwrap();
            sum += calories;
            continue;
        }

        if sum > max {
            max = sum;
        }

        sum = 0;
    }

    println!("Max calories: {:?}", max)
}
