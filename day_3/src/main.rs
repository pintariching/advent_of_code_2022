use std::fs;

static LOWERCASE_OFFSET: u8 = 96;
static UPPERCASE_OFFSET: u8 = 38;

fn main() {
    let input = fs::read_to_string("day_3/input.txt").unwrap();

    let mut rucksack: Vec<usize> = Vec::new();
    let mut group: Vec<usize> = Vec::new();

    let mut buf = Vec::new();
    for (index, line) in input.lines().enumerate() {
        let items = line.split_at(line.len() / 2);
        if let Some(common_char) = find_common_char(items.0, items.1) {
            rucksack.push(char_to_priority(&common_char));
        }

        buf.push(line);
        if (index + 1) % 3 == 0 {
            if let Some(common_group_char) = find_common_group_char(buf[0], buf[1], buf[2]) {
                group.push(char_to_priority(&common_group_char));
            }

            buf.clear();
        }
    }

    println!(
        "The sum of all the priorities of items that appear in both compartments is: {}",
        rucksack.iter().sum::<usize>()
    );

    println!(
        "The sum of all the priorities of items that appear in each three-Elf group: {}",
        group.iter().sum::<usize>()
    );
}

fn char_to_priority(char: &char) -> usize {
    if char.is_alphabetic() {
        if char.is_uppercase() {
            (*char as u8 - UPPERCASE_OFFSET) as usize
        } else {
            (*char as u8 - LOWERCASE_OFFSET) as usize
        }
    } else {
        0
    }
}

fn find_common_char(left: &str, right: &str) -> Option<char> {
    for char in left.chars() {
        if right.contains(char) {
            return Some(char);
        }
    }

    None
}

fn find_common_group_char(left: &str, middle: &str, right: &str) -> Option<char> {
    for char in left.chars() {
        if middle.contains(char) && right.contains(char) {
            return Some(char);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority(&'a'), 1);
        assert_eq!(char_to_priority(&'A'), 27);
        assert_eq!(char_to_priority(&'p'), 16);
        assert_eq!(char_to_priority(&'L'), 38);
    }

    #[test]
    fn test_common_chars() {
        assert_eq!(find_common_char("dWlhclDHd", "FvDCCDfFq"), Some('D'));
    }

    #[test]
    fn test_find_common_group_char() {
        assert_eq!(
            find_common_group_char(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ),
            Some('r')
        );

        assert_eq!(
            find_common_group_char(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ),
            Some('Z')
        );
    }
}
