use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug, Clone)]
struct Window(Vec<u64>);

impl Window {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn is_filled(&self) -> bool {
        self.0.len() >= 3
    }

    fn push(&mut self, elem: u64) {
        if !self.is_filled() {
            self.0.push(elem);
        }
    }

    fn sum(&self) -> u64 {
        self.0.iter().sum()
    }
}


fn main() {

    // part 1
    ///////////////////////////////////////////////////////////////////////////
    let f = OpenOptions::new()
                .read(true)
                .open("input").expect("input not present");
    let buf_read = BufReader::new(f);

    let mut count = 0;
    let mut pred = 0;
    for l in buf_read.lines() {
        let number = u64::from_str_radix(l.unwrap().as_str(), 10).unwrap();
        if  number > pred {
            count += 1;
        }
        pred = number;
    }

    println!("Number of times a depth measurement increases: {}", count-1);
    
    // part 2
    ///////////////////////////////////////////////////////////////////////////

    let f = OpenOptions::new()
                .read(true)
                .open("input").expect("input not present");
    let buf_read = BufReader::new(f);

    let mut count = 0;
    let mut a = Window(Vec::new());
    let mut b = Window(Vec::new());
    let mut c = Window(Vec::new());

    for l in buf_read.lines() {
        let number = u64::from_str_radix(l.unwrap().as_str(), 10).unwrap();
        
        if a.is_empty() {
            a.push(number);
        } else if b.is_empty() {
            a.push(number);
            b.push(number);
        } else {
            a.push(number);
            b.push(number);
            c.push(number);
        }

        if a.is_filled() && b.is_filled() {
            if b.sum() > a.sum() {
                count += 1;
            }
            a = b.clone();
            b = c.clone();
            c = Window(vec![number]);
        }
    }

    println!("Number of times the sum of measurements in this sliding window increases: {}", count);
    
}
