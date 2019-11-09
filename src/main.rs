fn calculated_average (vec_of_nums : &Vec<usize>, vec_lng : usize) -> usize {
    let mut sum_of_nums:usize = 0;

    for num in vec_of_nums {
        sum_of_nums = sum_of_nums + num;
    }

    sum_of_nums / vec_lng
}

fn main() {
    let mut vec_of_nums:Vec<usize> = vec![5, 1, 3, 8, 9];
    let vec_lng:usize = vec_of_nums.len();

    let half_index:usize = vec_lng / 2;

    &vec_of_nums.sort();
    let median = &vec_of_nums[half_index];

    let average = calculated_average(&vec_of_nums, vec_lng);


    println!("The average is {}, the median is {}, the half index is {}", average, median, half_index);
}
