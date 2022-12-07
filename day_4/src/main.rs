use std::fs::read_to_string;

fn main() {
    let input = read_to_string("day_4/input.txt").unwrap();

    let mut num_of_contained_ranges = 0;
    let mut num_of_overlapping_ranges = 0;

    for line in input.lines() {
        let (left, right) = line.split_once(",").unwrap();
        let left_range = str_to_range(left);
        let right_range = str_to_range(right);

        if contains_range(&left_range, &right_range) {
            num_of_contained_ranges += 1;
        }

        if overlaps_range(&left_range, &right_range) {
            num_of_overlapping_ranges += 1;
        }
    }

    println!(
        "The number of assignment pairs that fully contain the other is: {}",
        num_of_contained_ranges
    );

    println!(
        "The number of assignment pairs with overlapping ranges is: {}",
        num_of_overlapping_ranges
    );
}

fn str_to_range(value: &str) -> Vec<u8> {
    let (left, right) = value.split_once("-").unwrap();

    let left = left.parse::<u8>().unwrap();
    let right = right.parse::<u8>().unwrap();

    let mut out = Vec::new();
    for i in 0..(right - left) + 1 {
        out.push(left + i);
    }

    out
}

fn contains_range(left: &Vec<u8>, right: &Vec<u8>) -> bool {
    left.iter().all(|l| right.contains(l)) || right.iter().all(|r| left.contains(r))
}

fn overlaps_range(left: &Vec<u8>, right: &Vec<u8>) -> bool {
    left.iter().any(|l| right.contains(l))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_to_range() {
        assert_eq!(str_to_range("2-4"), vec![2, 3, 4]);
        assert_eq!(str_to_range("6-8"), vec![6, 7, 8]);
        assert_eq!(str_to_range("7-9"), vec![7, 8, 9]);
        assert_eq!(str_to_range("2-6"), vec![2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_contains_range() {
        assert!(contains_range(&vec![2, 3, 4], &vec![1, 2, 3, 4, 5]));
        assert!(contains_range(&vec![6, 7, 8, 9], &vec![6, 7]));
        assert!(!contains_range(&vec![1, 2, 3], &vec![2, 3, 4]));
    }

    #[test]
    fn test_overlap_range() {
        assert!(overlaps_range(&vec![2, 3, 4], &vec![4, 5, 6]));
        assert!(overlaps_range(&vec![5, 6], &vec![4, 5, 6, 7]));
        assert!(overlaps_range(&vec![1, 2, 3], &vec![2]));
        assert!(!overlaps_range(&vec![1, 2, 3], &vec![4, 5, 6]))
    }
}
