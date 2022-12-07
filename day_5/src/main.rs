use std::fs::read_to_string;

struct Instruction {
    from: usize,
    to: usize,
    amount: usize,
    crane_model: CraneMoverModel,
}

impl Instruction {
    fn new(line: &str, crane_model: CraneMoverModel) -> Self {
        let nums: Vec<usize> = line
            .split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        Self {
            from: nums[1] - 1,
            to: nums[2] - 1,
            amount: nums[0],
            crane_model,
        }
    }
}

#[derive(Clone, Copy)]
enum CraneMoverModel {
    CraneMover9000,
    CraneMover9001,
}

static CRANE_MOVER_MODEL: CraneMoverModel = CraneMoverModel::CraneMover9001;

fn main() {
    let input = read_to_string("day_5/input.txt").unwrap();

    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let instructions = instructions.trim();

    let mut crates = read_crates(crates);
    let instructions: Vec<Instruction> = instructions
        .lines()
        .map(|l| Instruction::new(l, CRANE_MOVER_MODEL))
        .collect();

    for instr in instructions {
        move_crates(instr, &mut crates);
    }

    let output_word: String = crates.iter().map(|col| col.last().unwrap()).collect();

    println!(
        "After the rearrangment, the crates on top of each stack are: {}",
        output_word
    );
}

fn read_crates(input: &str) -> Vec<Vec<char>> {
    let amount_of_cols = (input.lines().nth(0).unwrap().len() + 1) / 4;

    let mut out = Vec::new();
    for _ in 0..amount_of_cols {
        out.push(Vec::new());
    }

    for (index, line) in input.lines().rev().enumerate() {
        if index == 0 {
            continue;
        }

        for i in 0..amount_of_cols {
            let v = out.iter_mut().nth(i).unwrap();
            let char = line.chars().nth(1 + 4 * i).unwrap();

            if !char.is_whitespace() {
                v.push(line.chars().nth(1 + 4 * i).unwrap());
            }
        }
    }

    out
}

fn move_crates(instruction: Instruction, crates: &mut Vec<Vec<char>>) {
    let col_len = crates[instruction.from].len();
    let crates_to_move: &mut Vec<char> = &mut crates[instruction.from]
        .drain(col_len - instruction.amount..)
        .collect();

    match instruction.crane_model {
        CraneMoverModel::CraneMover9000 => crates_to_move.reverse(),
        CraneMoverModel::CraneMover9001 => (),
    }

    crates[instruction.to].append(crates_to_move);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_crates() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

        assert_eq!(
            read_crates(input),
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
        );
    }

    #[test]
    fn test_move_crates() {
        let input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#;

        let mut crates = read_crates(input);

        move_crates(
            Instruction {
                from: 0,
                to: 1,
                amount: 1,
                crane_model: CraneMoverModel::CraneMover9000,
            },
            &mut crates,
        );

        assert_eq!(crates, vec![vec!['Z'], vec!['M', 'C', 'D', 'N'], vec!['P']]);

        move_crates(
            Instruction {
                from: 1,
                to: 0,
                amount: 3,
                crane_model: CraneMoverModel::CraneMover9000,
            },
            &mut crates,
        );

        assert_eq!(crates, vec![vec!['Z', 'N', 'D', 'C'], vec!['M'], vec!['P']]);

        move_crates(
            Instruction {
                from: 0,
                to: 2,
                amount: 3,
                crane_model: CraneMoverModel::CraneMover9001,
            },
            &mut crates,
        );

        assert_eq!(crates, vec![vec!['Z'], vec!['M'], vec!['P', 'N', 'D', 'C']]);
    }
}
