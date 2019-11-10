use std::collections::HashMap;

mod simple_calculators;

fn build_num_frequencies (vec_of_nums : &Vec<usize>) -> HashMap<usize, i32>{
    let mut num_frequencies = HashMap::new();

    for &num in vec_of_nums {
        let counter = num_frequencies.entry(num).or_insert(0);
        *counter += 1;
    }

    num_frequencies
}

pub fn trigger_main () {
    let mut vec_of_nums:Vec<usize> = vec![5, 1, 5, 8, 9];
    let vec_lng:usize = vec_of_nums.len();
    let half_index:usize = vec_lng / 2;

    &vec_of_nums.sort();
    let median = &vec_of_nums[half_index];

    let average = simple_calculators::calculate_average(&vec_of_nums, vec_lng);
    
    let num_frequencies = build_num_frequencies(&vec_of_nums);

    let highest_freq:Option<&i32> = Some(&0);

    let mode = simple_calculators::calculate_mode(&vec_of_nums, &num_frequencies, highest_freq);

    println!("The average is {}, the median is {}, the mode is {:?}", average, median, mode);

}