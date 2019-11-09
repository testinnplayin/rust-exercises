use std::collections::HashMap;

fn calculated_average (vec_of_nums : &Vec<usize>, vec_lng : usize) -> usize {
    let mut sum_of_nums:usize = 0;

    for num in vec_of_nums {
        sum_of_nums = sum_of_nums + num;
    }

    sum_of_nums / vec_lng
}

fn calculate_mode <'a> (vec_of_nums : &Vec<usize>, num_frequencies : &'a HashMap<usize, i32>, mut highest_freq : Option<&'a i32>) -> Option<&'a i32> {
    for num in vec_of_nums {
        let freq = num_frequencies.get(&num);

        if highest_freq < freq {
            highest_freq = freq;
        }
    }

    highest_freq
}

fn main() {
    let mut vec_of_nums:Vec<usize> = vec![5, 1, 5, 8, 9];
    let vec_lng:usize = vec_of_nums.len();

    let half_index:usize = vec_lng / 2;

    &vec_of_nums.sort();
    let median = &vec_of_nums[half_index];

    let average = calculated_average(&vec_of_nums, vec_lng);
    let mut num_frequencies = HashMap::new();

    for &num in &vec_of_nums {
        let counter = num_frequencies.entry(num).or_insert(0);
        *counter += 1;
    }

    let highest_freq:Option<&i32> = Some(&0);

    let mode = calculate_mode(&vec_of_nums, &num_frequencies, highest_freq);

    println!("The average is {}, the median is {}, the mode is {:?}", average, median, mode);
}
