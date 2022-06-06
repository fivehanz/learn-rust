use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // let numbers: Vec<i32> = vec![1, 7, 2, 4, 3, 5, 8, 6, 9, 10, 11, 12];
    let numbers = vec![1, 1, 1, 3, 4, 5, 2, 4, 6, 0];

    println!("{:?} mean: {}", numbers, mean(&numbers));
    println!("{:?} median: {}", numbers, median(&numbers));
    println!("{:?} mode: {}", numbers, mode(&numbers));
}

fn mean(numbers: &[i32]) -> f64 {
    // sum the numbers, divide by the length of the vector

    // let mut sum: f64 = 0.0;
    // for num in numbers {
    //     sum += *num as f64;
    // }

    // sum / numbers.len() as f64

    let sum = numbers.iter().fold(0, |a, b| a + b);
    sum as f64 / numbers.len() as f64
}

fn median(numbers: &[i32]) -> f64 {
    // sort the array, return the middle number.
    let mut sorted_numbers = numbers.to_vec();
    sorted_numbers.sort();

    let middle = sorted_numbers.len() / 2;

    // if len is even; return the mean of the two middle numbers
    if sorted_numbers.len() % 2 == 0 {
        return mean(&[sorted_numbers[middle], sorted_numbers[middle - 1]]);
    }

    // else return middle number
    sorted_numbers[middle] as f64
}

fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut max_value = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > max_value {
            max_value = value;
            mode = *key;
        }
    }

    mode
}
