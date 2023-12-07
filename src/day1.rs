use std::{fs, prelude::*, io::{BufReader, BufRead}, vec, iter::Map};

pub fn Trebuchet_pt1 (){
    if let Ok(read_file) = fs::File::open("src/files/day1.txt") {
        println!("\nOpening Day 1 text file success\n");

        let reader = BufReader::new(read_file);
        let mut sum: u32 = 0;
        let nums_as_chars = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mut num_str = String::new();

            // push first found num to string
            num_str.push(line.chars().find(|c| nums_as_chars.contains(&c)).unwrap());

            // push last found num to string
            num_str.push(line.chars().rfind(|c| nums_as_chars.contains(&c)).unwrap());
            sum += num_str.parse::<u32>().unwrap();
        }

        println!("Interger sum: {sum}")


    }else {
        println!("\nday1.txt not found\n");
        return;
    }
}

pub fn Trebuchet_pt2() {
    if let Ok(read_file) = fs::File::open("src/files/day1.txt") {
        println!("\nOpening Day 1 text file success\n");

        let reader = BufReader::new(read_file);
        let mut sum: u32 = 0;
        let nums_as_strings = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                                                "1", "2", "3", "4", "5", "6", "7", "8", "9"];

       
        println!("Integer and text sum: {sum}")


    }else {
        println!("\nday1.txt not found\n");
        return;
    }
}