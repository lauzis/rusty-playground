extern crate regex;
use std::fs;
// use std::str;
use regex::Regex;

fn main() {


    /// 1. reading file with strings
    /// 2. going line by line
    /// 3. with regex getting first match
    /// 4. reversing string and again getting first number match
    /// so we get first and last number
    /// 5. concatenate first and last
    /// 6. then parsing as int
    /// 7. sum the numbers
    /// 8. printing out result
    /// -----------------------
    /// well it could be actually if we get all matches at once, without reversing string
    /// but as i am new to the rust it felt easier way to solve it
    /// but if we get all matches of numbers we could take vecor/array first and last element
    /// concatenate them and then do the same thing
    ///
    ///
    /// this is my first try with rust, felt like a black box a bit.
    ///
    /// Todo from this task
    /// - need to know more about types
    /// - need to know more abnout type conversion
    /// - need to know how to read file line by line

    let re = Regex::new(r"[0-9]").unwrap();

    let file_path = "./input.txt";
    println!("In file {}", file_path);

    // we got the data in string, looks  like we get all at once
    // maybe there is better way to read line by line, on other hand
    // its probably faster to read everything at once
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    // splitting the read file into lines
    let lines = contents.split("\n");


    // some varaibles for manipulation
    let mut reverse_line:String;
    let mut sum = 0;
    let mut first_number_string = "";
    let mut last_number_string = "";
    let mut combined_numbner_string: String = "".to_string();

    let mut number = 0;

    for line in lines {
        let Some(caps) = re.captures(line) else {
            println!("no match!");
            return;
        };

        first_number_string = &caps[0];

        reverse_line = line.to_string().chars().rev().collect::<String>();
        let Some(caps) = re.captures(&reverse_line) else {
            println!("no match!");
            return;
        };
        last_number_string = &caps[0];
        combined_numbner_string = first_number_string.to_owned() + last_number_string;
        number = combined_numbner_string.parse().unwrap();
        sum += number;

        println!("{}", sum);
    }

}