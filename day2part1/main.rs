use std::fs;

fn create_vec_newline(text_input: String) -> Vec<String>{
    let vec_of_commands: Vec<String> = text_input
        .lines()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    return vec_of_commands;
}

fn create_vec_white(text_input: &String) -> Vec<String>{
    let vec_of_commands: Vec<String> = text_input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    return vec_of_commands;
}

fn horizontal_position(text_input: &Vec<String>) -> i32{
    let mut position = 0;
    for line in text_input{
        let command = create_vec_white(line);
        if command[0] == "forward"{
            let number: i32 = command[1].parse().unwrap();
            position = position + number;
        }else {
            continue
        }
    }return position;
}

fn depth_position(text_input: &Vec<String>) -> i32{
    let mut depth = 0;
    for line in text_input{
        let command = create_vec_white(line);
        if command[0] == "down" {
            let number: i32 = command[1].parse().unwrap();
            depth = depth + number;
        }else if command[0] == "up"{
            let number: i32 = command[1].parse().unwrap();
            depth = depth - number;
        }
    }return depth;
}

fn main() {
    let raw_input = fs::read_to_string("/home/paul/IdeaProjects/day2part1/src/input.txt").expect("Unable to read file");
    let output = create_vec_newline(raw_input);
    let h_position = horizontal_position(&output);
    let depth = depth_position(&output);
    println!("The horizontal postion is {}",h_position);
    println!("The depth is {}",depth);
    println!("The answer is {}",h_position * depth);

}