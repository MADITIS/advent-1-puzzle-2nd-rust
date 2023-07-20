// use std::env;
use std::fs;

fn main() {
    let file_path = "./src/data.txt";
    println!("In file {}", file_path);
    // reading a file using fs module and function read_to_string
    let contents = fs::read_to_string(file_path).expect("Some Error happened");

    // unused warning because not "using" it. just assigning.
    let mut values: Vec<i64> = Vec::new();

    let mut sum: i64 = 0;
    for value in contents.lines() {
        if value != "" {
            let number: i64 = value.parse().expect("Number was expected");
            sum += number;
        } else {
            values.push(sum);
            sum = 0;
        }
    }

    values.sort();
    values.reverse();
    println!("{values:?}");

    let mut max_value: i64 = 0;
    for i in &values[0..3] {
        println!("{i}");
        max_value += i;
    }



    println!("{max_value:?}");

}

