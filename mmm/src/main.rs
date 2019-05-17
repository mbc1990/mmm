use std::io;
use std::io::prelude::*;

fn mean(input: &Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for num in input.iter() {
       sum += num;
    }
    return (sum / input.len() as f64) as f64;
}

fn median(input: &Vec<f64>) -> f64 {
    let mut to_sort = input.clone();
    to_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return to_sort[to_sort.len() / 2];
}

fn min_max(input: &Vec<f64>) -> (f64, f64) {
    let mut to_sort = input.clone();
    to_sort.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return (to_sort[0], to_sort[to_sort.len()-1]);
}

fn sum(input: &Vec<f64>) -> f64 {
    input.iter().sum()
}


fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut input = Vec::new();
    for line in stdin.lock().lines() {
        let line_input = line.expect("Unable to get stdin line");
        let num = line_input.parse::<f64>().expect("Only f64 is currently supported");
        input.push(num);
    }
    let mean = mean(&input);
    let median = median(&input);
    let (min_val, max_val) = min_max(&input);
    let sum = sum(&input);
    println!("mean: {:?}", mean);
    println!("median: {:?}", median);
    println!("min: {:?}", min_val);
    println!("max: {:?}", max_val);
    println!("sum: {:?}", sum);
    Ok(())
}
