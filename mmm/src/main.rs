use std::io;
use std::io::prelude::*;

fn mean(input: &Vec<i64>) -> i64 {
    let mut sum = 0;
    for num in input.iter() {
       sum += num;
    }
    return (sum / input.len() as i64) as i64;
}

fn median(input: &Vec<i64>) -> i64 {
    let mut to_sort = input.clone();
    to_sort.sort();
    return to_sort[to_sort.len() / 2];
}

fn min_max(input: &Vec<i64>) -> (i64, i64) {
    let mut to_sort = input.clone();
    to_sort.sort();
    return (to_sort[0], to_sort[to_sort.len()-1]);
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let line_input = line.expect("Unable to get stdin line");
        let num = line_input.parse::<i64>().expect("Only i64 is currently supported");
        input.push(num);
    }
    let mean = mean(&input);
    let median = median(&input);
    let (min_val, max_val) = min_max(&input);
    println!("mean: {:?}", mean);
    println!("median: {:?}", median);
    println!("min: {:?}", min_val);
    println!("max: {:?}", max_val);
    Ok(())
}
