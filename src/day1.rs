use std::{
    fs,
    io::{BufRead, BufReader},
    vec,
};

pub fn trebuchet_pt1() {
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
    } else {
        println!("\nday1.txt not found\n");
        return;
    }
}

pub fn trebuchet_pt2() {
    if let Ok(read_file) = fs::File::open("src/files/day1.txt") {
        println!("\nOpening Day 1 text file success\n");

        let reader = BufReader::new(read_file);
        let mut sum: u32 = 0;
        let nums_as_strings = vec![
            ["one", "1"],
            ["two", "2"],
            ["three", "3"],
            ["four", "4"],
            ["five", "5"],
            ["six", "6"],
            ["seven", "7"],
            ["eight", "8"],
            ["nine", "9"],
        ];

        for line_result in reader.lines() {
            let line = line_result.unwrap();
            let mut num_str = String::new();

            num_str.push_str(word_to_num(
                find_first_occurence_of_str(&nums_as_strings, &line),
                &nums_as_strings,
            ));
            num_str.push_str(word_to_num(
                find_last_occurence_of_str(&nums_as_strings, &line),
                &nums_as_strings,
            ));

            sum += num_str.parse::<u32>().unwrap();
        }
        println!("Integer and text sum: {sum}")
    } else {
        println!("\nday1.txt not found\n");
        return;
    }
}

fn word_to_num<'a>(word: &'a str, nums_as_strings: &'a Vec<[&str; 2]>) -> &'a str {
    let num = nums_as_strings
        .iter()
        .find_map(|&[s1, s2]| if word == s1 { Some(s2) } else { None });

    match num {
        Some(num) => num,
        None => word,
    }
}

fn find_first_occurence_of_str<'a>(search_for: &'a Vec<[&str; 2]>, search_in: &str) -> &'a str {
    search_for
        .iter()
        .filter_map(|&[s1, s2]| {
            let index_s1 = search_in.find(s1);
            let index_s2 = search_in.find(s2);

            match (index_s1, index_s2) {
                (Some(idx1), Some(idx2)) => Some(if idx1 < idx2 { (idx1, s1) } else { (idx2, s2) }),
                (Some(idx1), None) => Some((idx1, s1)),
                (None, Some(idx2)) => Some((idx2, s2)),
                (None, None) => None,
            }
        })
        .min_by_key(|&(index, _)| index)
        .map(|(_, s)| s)
        .unwrap()
}

fn find_last_occurence_of_str<'a>(search_for: &'a Vec<[&str; 2]>, search_in: &str) -> &'a str {
    search_for
        .iter()
        .filter_map(|&[s1, s2]| {
            let index_s1 = search_in.rfind(s1);
            let index_s2 = search_in.rfind(s2);

            match (index_s1, index_s2) {
                (Some(idx1), Some(idx2)) => Some(if idx1 > idx2 { (idx1, s1) } else { (idx2, s2) }),
                (Some(idx1), None) => Some((idx1, s1)),
                (None, Some(idx2)) => Some((idx2, s2)),
                (None, None) => None,
            }
        })
        .max_by_key(|&(index, _)| index)
        .map(|(_, s)| s)
        .unwrap()
}
