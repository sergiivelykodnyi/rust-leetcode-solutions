// 1480. Running Sum of 1d Array
// Difficulty: Easy
// Link: https://leetcode.com/problems/running-sum-of-1d-array/
// Description:
// Given an array of integers "numbers", return the running sum of the array as runningSum[i] = sum(numbers[0]numbers[i]).
// Return the running sum of numbers.

pub fn running_sum(numbers: Vec<i32>) -> Vec<i32> {
    let mut res:Vec<i32> = Vec::new();
    let mut sum = 0;

    for value in numbers {
        sum += value;
        res.push(sum);
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(vec![1,2,3,4]), vec![1,3,6,10]);
        assert_eq!(running_sum(vec![1,1,1,1,1]), vec![1,2,3,4,5]);
        assert_eq!(running_sum(vec![3,1,2,10,1]), vec![3,4,6,16,17]);
    }
}
