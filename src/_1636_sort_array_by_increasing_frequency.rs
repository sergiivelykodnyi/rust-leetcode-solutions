// Problem: 1636. Sort Array by Increasing Frequency
// Difficulty: Easy
// Link: https://leetcode.com/problems/sort-array-by-increasing-frequency/
// Description:
// Given an array of integers "numbers", sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.

use std::collections::HashMap;

pub fn frequency_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    for n in numbers {
        // One line solution:
        // frequency_map.entry(n).and_modify(|n: &mut i32| *n += 1).or_insert(1);

        if frequency_map.contains_key(&n) {
            let count = frequency_map.get_mut(&n).unwrap();
            *count += 1;
        } else {
            frequency_map.insert(n, 1);
        }
    }

    let mut frequency_entries: Vec<(&i32, &i32)> = Vec::new();

    for (key, value) in frequency_map.iter() {
        frequency_entries.push((key, value));
    }

    frequency_entries.sort_by(|a, b| {
        if a.1 == b.1 {
            b.0.cmp(a.0)
        } else {
            a.1.cmp(b.1)
        }
    });

    let mut frequency_array: Vec<i32> = Vec::new();

    for (num, frequency) in frequency_entries {
        for _ in 0..*frequency {
            frequency_array.push(*num);
        }
    }

    frequency_array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(frequency_sort(vec![1,1,2,2,2,3]), vec![3,1,1,2,2,2]);
        assert_eq!(frequency_sort(vec![2,3,1,3,2]), vec![1,3,3,2,2]);
        assert_eq!(frequency_sort(vec![-1,1,-6,4,5,-6,1,4,1]), vec![5,-1,4,4,-6,-6,1,1,1]);
    }
}
