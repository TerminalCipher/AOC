use std::fs;

fn value_increase(nums: Vec<i32>) -> i32{
   let mut total = 0;
   let mut previous = 0;
   let mut current = 0;

   for num in nums {
      current = num;
      if current > previous{
         total = total + 1;
         previous = current;
      }else {
         previous = current;
      }
   }

   let final_count = total -1;
   return final_count
}

fn main() {
   let raw_input = fs::read_to_string("/home/paul/IdeaProjects/day1/src/input.txt").expect("Unable to read file");
   let numbers: Vec<i32> = raw_input
       .split_whitespace()
       .map(|s| s.parse().expect("parse error"))
       .collect();

   let count = value_increase(numbers);
   println!("{}", count);
}