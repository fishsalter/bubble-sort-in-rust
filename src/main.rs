use std::io;
use bubble_sort_in_rust::sort;

fn main() {
    println!("input numbers(separated by spaces): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can not read numbers");

    let mut numbers: Vec<f64> = input.split_whitespace().map(|x| x.parse::<f64>().expect("please input number")).collect();

    sort::bubble_sort(&mut numbers);

    let result: Vec<String>= numbers.iter().map(|x| {x.to_string()}).collect();

    println!("{:?}", result);
}
