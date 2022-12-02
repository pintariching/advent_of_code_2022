use std::fs;

fn main() {
    let input = fs::read_to_string("day_1/input.txt").unwrap();

    let mut max_calories = 0;
    let mut elves = Vec::new();
    for s in input.split("\n\n") {
        let foods = s.split('\n');

        let parsed_foods = foods
            .map(|f| f.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let sum: usize = parsed_foods.iter().sum();
        if sum > max_calories {
            max_calories = sum;
        }

        elves.push(sum);
    }

    println!("The max calories an elf has is: {}", max_calories);

    elves.sort();

    let len = elves.len();
    let top_three = elves[len - 1] + elves[len - 2] + elves[len - 3];
    println!(
        "The top three elves are carying a total of {} calories",
        top_three
    );
}
