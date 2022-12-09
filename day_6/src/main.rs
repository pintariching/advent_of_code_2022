use std::fs;

static REQUIRED_UNIQUE_POSITIONS: usize = 14;

fn main() {
    let input = fs::read_to_string("day_6/input.txt").unwrap();

    let mut marker_index = 0;
    for i in 0..input.len() {
        if let Some(slice) = input.get(i..i + REQUIRED_UNIQUE_POSITIONS) {
            if is_unique(slice.chars().collect()) {
                marker_index = i + REQUIRED_UNIQUE_POSITIONS;
                break;
            }
        }
    }

    println!("The marker is at the index: {}", marker_index);
}

fn is_unique(slice: Vec<char>) -> bool {
    for char in &slice {
        let mut repeat_count = 0;
        for c in &slice {
            if *c == *char {
                repeat_count += 1
            }
        }

        if repeat_count > 1 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert!(is_unique(vec!['a', 'b', 'c', 'd']));
        assert!(!is_unique(vec!['a', 'a', 'c', 'd']));
    }
}
