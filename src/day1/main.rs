use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn get_ints(line: &str) -> i32 {
    let first_digit: String = get_digit(line, true);
    let last_digit: String = get_digit(line, false);
    println!("{first} {last}", first = first_digit, last = last_digit);

    return (first_digit.to_owned() + last_digit.as_str())
        .parse::<i32>()
        .unwrap();
}

fn get_digit(line: &str, first: bool) -> String {
    let re = Regex::new(r"\d|zero|one|two|three|four|five|six|seven|eight|nine").unwrap();
    for i in 0..line.len() {
        let ss: String;
        if first {
            ss = line.chars().take(i + 1).collect::<String>();
        } else {
            ss = line
                .chars()
                .skip(line.len() - i - 1)
                .take(i + 1)
                .collect::<String>();
        }
        println!("Substring: {}", ss);
        let digit: Option<&str> = re.find(&ss).map(|x| x.as_str());
        match digit {
            None => continue,
            Some("zero") => return "0".to_owned(),
            Some("one") => return "1".to_owned(),
            Some("two") => return "2".to_owned(),
            Some("three") => return "3".to_owned(),
            Some("four") => return "4".to_owned(),
            Some("five") => return "5".to_owned(),
            Some("six") => return "6".to_owned(),
            Some("seven") => return "7".to_owned(),
            Some("eight") => return "8".to_owned(),
            Some("nine") => return "9".to_owned(),
            Some(&_) => return digit.unwrap().to_string(),
        }
    }
    return "failed".to_owned();
}

fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");

    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    let _result = buf_reader.read_to_string(&mut contents);
    let mut value: i32 = 0;
    for line in contents.lines() {
        value += get_ints(line);
    }
    println!("{}", value);
    Ok(())
}
