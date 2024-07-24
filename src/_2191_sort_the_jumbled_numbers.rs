// Problem: 2191. Sort the Jumbled Numbers
// Difficulty: Medium
// Link: https://leetcode.com/problems/sort-the-jumbled-numbers
// Description:
// You are given a 0-indexed integer array mapping which represents the mapping
// rule of a shuffled decimal system. mapping[i] = j means digit i should be
// mapped to digit j in this system.
// The mapped value of an integer is the new integer obtained by replacing each
// occurrence of digit i in the integer with mapping[i] for all 0 <= i <= 9.
// You are also given another integer array numbers. Return the array numbers
// sorted in non-decreasing order based on the mapped values of its elements.

fn transform_number_to_digits(n: i32) -> Vec<i32> {
    if n == 0 {
        return vec![0]
    }

    let mut digits: Vec<i32> = Vec::new();
    let mut n: i32 = n;

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits.reverse();

    digits
}

fn transform_digits_to_number(digits: Vec<i32>) -> i32 {
    let mut n: i32 = 0;

    for i in digits {
        n = n * 10 + i;
    }

    n
}

fn transform_number_to_mapped_number(n: i32, mapping: Vec<i32>) -> i32 {
    let digits: Vec<i32> = transform_number_to_digits(n);
    let mut mapped_digits: Vec<i32> = Vec::new();

    for d in digits {
        mapped_digits.push(mapping[d as usize]);
    }

    transform_digits_to_number(mapped_digits)
}

pub fn sort_jumbled(mapping: Vec<i32>, numbers: Vec<i32>) -> Vec<i32> {
    let mut map: Vec<(i32, i32)> = Vec::new();

    for number in numbers {
        let mapped_number = transform_number_to_mapped_number(number, mapping.to_vec());

        map.push((number, mapped_number));
    }

    map.sort_by(|a: &(i32, i32), b: &(i32, i32)| {(*a).1.cmp(&b.1)});

    let mut result: Vec<i32> = Vec::new();

    for entry in map {
        result.push(entry.0);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_number_to_digits() {
        assert_eq!(transform_number_to_digits(991), vec![9,9,1]);
        assert_eq!(transform_number_to_digits(338), vec![3,3,8]);
        assert_eq!(transform_number_to_digits(38), vec![3,8]);
    }

    #[test]
    fn test_transform_digits_to_number() {
        assert_eq!(transform_digits_to_number(vec![9,9,1]), 991);
        assert_eq!(transform_digits_to_number(vec![3,3,8]), 338);
        assert_eq!(transform_digits_to_number(vec![3,8]), 38);
    }

    #[test]
    fn test_transform_number_to_mapped_number() {
        assert_eq!(transform_number_to_mapped_number(991, vec![9,8,7,6,5,4,3,2,1,0]), 8);
        assert_eq!(transform_number_to_mapped_number(338, vec![9,8,7,6,5,4,3,2,1,0]), 661);
        assert_eq!(transform_number_to_mapped_number(38, vec![9,8,7,6,5,4,3,2,1,0]), 61);
    }

    #[test]
    fn test_sort_jumbled() {
        assert_eq!(sort_jumbled(vec![9,8,7,6,5,4,3,2,1,0], vec![0,1,2,3,4,5,6,7,8,9]), vec![9,8,7,6,5,4,3,2,1,0]);
        assert_eq!(sort_jumbled(vec![0,1,2,3,4,5,6,7,8,9], vec![789,456,123]), vec![123,456,789]);
        assert_eq!(sort_jumbled(vec![8,9,4,0,2,1,3,5,7,6], vec![991,338,38]), vec![338,38,991]);
        assert_eq!(sort_jumbled(vec![9,8,7,6,5,4,3,2,1,0], vec![9, 99, 999, 9999, 99999]), vec![9, 99, 999, 9999, 99999]);
    }
}
