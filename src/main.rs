use std::{fs, prelude::*, io::{BufReader, BufRead}, vec, iter::Map};

fn main() {
    //Day 1: Trebuchet?!

    if let Ok(read_file) = fs::File::open("src/files/day1.txt") {
        println!("\nOpening Day 1 text file success\n");

        let reader = BufReader::new(read_file);
        let mut sum: u32 = 0;
        let nums_as_strs = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mut num_str = String::new();

            // push first found num to string
            num_str.push(line.chars().find(|c| nums_as_strs.contains(&c)).unwrap());

            // push last found num to string
            num_str.push(line.chars().rfind(|c| nums_as_strs.contains(&c)).unwrap());
            sum += num_str.parse::<u32>().unwrap();
        }

        println!("{sum}")


    }else {
        println!("\nday1.txt not found\n");
        return;
    }


}
