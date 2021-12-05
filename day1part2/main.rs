use std::fs;

fn value_increase_blocks(nums: Vec<i32>) -> i32{
    let mut total = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut current_block = 0;
    let mut previous_block = 0;
    let mut count = 0;

    for num in nums {
        if count == 0{
            a = num;
            count = count + 1;
        }else if count == 1 {
            b = num;
            count = count + 1;
        }else if count == 2 {
            c = num;
            count = count +1;
        }else if count == 3 {
            previous_block = a + b + c;
            a = b;
            b = c;
            c = num;
            current_block = a + b + c;
            if current_block > previous_block{
                total = total + 1;
            }
        }
    }

    let final_count = total;
    return final_count
}

fn main() {
    let raw_input = fs::read_to_string("/home/paul/IdeaProjects/day1/src/input.txt").expect("Unable to read file");
    let numbers: Vec<i32> = raw_input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    let count = value_increase_blocks(numbers);
    println!("{}", count);
}