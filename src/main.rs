use std::fs;

fn main() {
    println!("Hello, world!");
    let file_path = "../day1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have read the file");

    let lines = contents.lines();
    let mut sum: i32 = 0;

    lines.for_each(|line| {
        let numerics: Vec<&str> = line.matches(char::is_numeric).collect();
        let numbers: Vec<i32> = numerics.iter().map(|c| c.parse::<i32>().unwrap()).collect();

        let first_num = numbers.first();
        let last_num = numbers.last();
        let combined_num = first_num.unwrap() * 10 + last_num.unwrap();

        sum += combined_num;
    });

    println!("sum: {sum}");
}
