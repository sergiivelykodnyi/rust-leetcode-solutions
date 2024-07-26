pub fn sort_array(mut numbers: Vec<i32>) -> Vec<i32> {
    // let mut sorted_numbers = numbers;

    loop {
        let mut swapped = false;

        for i in 0..numbers.len() - 1 {
            if numbers[i] > numbers[i + 1] {
                numbers.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    numbers
}

fn merge (mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();

    while !left.is_empty() && !right.is_empty() {
        if left[0] < right[0] {
            result.push(left.remove(0));
        } else {
            result.push(right.remove(0));
        }
    }

    result.extend(left);
    result.extend(right);

    result
}

pub fn merge_sort(numbers: Vec<i32>) -> Vec<i32> {
    if numbers.len() <= 1 {
        return numbers;
    }

    let mid = numbers.len() / 2;

    let left = merge_sort(numbers[..mid].to_vec());
    let right = merge_sort(numbers[mid..].to_vec());

    merge(left, right)
}

fn sift_down(numbers: &mut Vec<i32>, start: usize, end: usize) {
    let mut root = start;

    loop {
        let mut child = root * 2 + 1;

        if child > end {
            break;
        }

        if child < end && numbers[child] < numbers[child + 1] {
            child += 1;
        }

        if numbers[root] < numbers[child] {
            numbers.swap(root, child);
            root = child;
        } else {
            break;
        }
    }
}

pub fn heap_sort(numbers: Vec<i32>) -> Vec<i32> {
    let mut numbers = numbers;
    let end = numbers.len();

    for start in (0..end / 2).rev() {
        sift_down(&mut numbers, start, end - 1);
    }

    for end in (1..numbers.len()).rev() {
        numbers.swap(end, 0);
        sift_down(&mut numbers, 0, end - 1);
    }

    numbers.to_vec()
}
