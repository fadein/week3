use std::io::{self};

fn main() {
    let mut numbers = Vec::new();
    println!("Enter ten numbers:");
    for _ in 0..10 {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read input");
        let value: f64 = line.trim().parse().expect("Invalid number");
        numbers.push(value);
    }

    let mean: f64 = numbers.iter().sum::<f64>() / numbers.len() as f64;
    let variance: f64 = numbers
        .iter()
        .map(|&x| (x - mean).powi(2))
        .sum::<f64>()
        / (numbers.len() as f64 - 1.0);
    let std_dev = variance.sqrt();

    println!("The mean is {:.2}", mean);
    println!("The standard deviation is {:.5}", std_dev);
}

