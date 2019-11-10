use std::collections::HashMap;

pub fn calculate_average (vec_of_nums : &Vec<usize>, vec_lng : usize) -> usize {
    let mut sum_of_nums:usize = 0;

    for num in vec_of_nums {
        sum_of_nums = sum_of_nums + num;
    }

    sum_of_nums / vec_lng
}

pub fn calculate_mode <'a> (vec_of_nums : &Vec<usize>, num_frequencies : &'a HashMap<usize, i32>, mut highest_freq : Option<&'a i32>) -> Option<&'a i32> {
    for num in vec_of_nums {
        let freq = num_frequencies.get(&num);

        if highest_freq < freq {
            highest_freq = freq;
        }
    }

    highest_freq
}