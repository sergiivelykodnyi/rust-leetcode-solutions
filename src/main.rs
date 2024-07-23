mod _1480_running_sum_of_1d_array;
mod _1636_sort_array_by_increasing_frequency;


fn main() {
    println!("Problem: 1480. Running Sum of 1d Array.");
    println!("{:?}", _1480_running_sum_of_1d_array::running_sum(vec![1,2,3,4]));
    println!("{:?}", _1480_running_sum_of_1d_array::running_sum(vec![1,1,1,1,1]));
    println!("{:?}", _1480_running_sum_of_1d_array::running_sum(vec![3,1,2,10,1]));

    println!("Problem: 1636. Sort Array by Increasing Frequency.");
    println!("{:?}", _1636_sort_array_by_increasing_frequency::frequency_sort(vec![1,1,2,2,2,3]));
    println!("{:?}", _1636_sort_array_by_increasing_frequency::frequency_sort(vec![2,3,1,3,2]));
    println!("{:?}", _1636_sort_array_by_increasing_frequency::frequency_sort(vec![-1,1,-6,4,5,-6,1,4,1]));
}
