/* #[path="_1480_running_sum_of_1d_array.rs"]
mod module;

fn main() {
    println!("Problem: 1480. Running Sum of 1d Array.");
    println!("{:?}", module::running_sum(vec![1,2,3,4]));
    println!("{:?}", module::running_sum(vec![1,1,1,1,1]));
    println!("{:?}", module::running_sum(vec![3,1,2,10,1]));
} */

/* #[path="_1636_sort_array_by_increasing_frequency.rs"]
mod module;

fn main() {
    println!("Problem: 1636. Sort Array by Increasing Frequency.");
    println!("{:?}", module::frequency_sort(vec![1,1,2,2,2,3]));
    println!("{:?}", module::frequency_sort(vec![2,3,1,3,2]));
    println!("{:?}", module::frequency_sort(vec![-1,1,-6,4,5,-6,1,4,1]));
} */

#[path="_2191_sort_the_jumbled_numbers.rs"]
mod solution;

fn main() {
    let mapping: Vec<i32> = vec![9,8,7,6,5,4,3,2,1,0];
    let numbers: Vec<i32> = vec![9, 99, 999, 9999, 99999];

    println!("{:?}", solution::sort_jumbled(mapping, numbers));
}
