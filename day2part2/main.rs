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

fn find_position(text_input: &Vec<String>) -> i32{
    let mut h_position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in text_input{
        let command = create_vec_white(line);
        if command[0] == "forward"{
            let number: i32 = command[1].parse().unwrap();
            h_position = h_position + number;
            depth = depth + (aim * number);
        }else if command[0] == "down" {
            let number: i32 = command[1].parse().unwrap();
            aim = aim + number;
        }else if command[0] == "up"{
            let number: i32 = command[1].parse().unwrap();
            aim = aim - number;
        }
    }
    let position= h_position * depth;
    return position;
}

fn main() {
    let raw_input = fs::read_to_string("/home/paul/IdeaProjects/day2part1/src/input.txt").expect("Unable to read file");
    let output = create_vec_newline(raw_input);
    let h_position = find_position(&output);
    println!("The position is {}",h_position);

}