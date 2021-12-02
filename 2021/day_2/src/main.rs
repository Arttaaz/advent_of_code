use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader };

fn main() {

    // part 1
    let f = OpenOptions::new()
                .read(true)
                .open("input").expect("input not present");
    let buf_read = BufReader::new(f);

    let mut horizontal_pos = 0;
    let mut vertical_pos = 0;

    for l in buf_read.lines() {
        let l = l.unwrap();
        let mut command = l.split_whitespace();

        match command.next() {
            Some(s) =>
                match s {
                    "forward" => horizontal_pos += i64::from_str_radix(command.next().unwrap(), 10).unwrap(),
                    "down" => vertical_pos += i64::from_str_radix(command.next().unwrap(), 10).unwrap(),
                    "up" => vertical_pos -= i64::from_str_radix(command.next().unwrap(), 10).unwrap(),
                    _ => unreachable!(),
                },
            None => (),
        }
    }

    println!("result = {}", horizontal_pos * vertical_pos);

    // part 2
    let f = OpenOptions::new()
                .read(true)
                .open("input").expect("input not present");
    let buf_read = BufReader::new(f);

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for l in buf_read.lines() {
        let l = l.unwrap();
        let mut command = l.split_whitespace();

        match command.next() {
            Some(s) =>
                match s {
                    "forward" => {
                        let number = i64::from_str_radix(command.next().unwrap(), 10).unwrap();
                        horizontal_pos += number;
                        depth += number * aim;
                    },
                    "down" => aim += i64::from_str_radix(command.next().unwrap(), 10).unwrap(),
                    "up" => aim -= i64::from_str_radix(command.next().unwrap(), 10).unwrap(),
                    _ => unreachable!(),
                },
            None => (),
        }
    }

    println!("result = {}", horizontal_pos * depth);
}
