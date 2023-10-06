use std::fs::{self, File};
use std::io::prelude::*;

/// Takes a list of numbers and returns the result of adding them together
///
/// # Arguments
/// * numbers - A Vec of numbers
///
/// # Examples
/// ```
/// // Returns 15
/// add_some_numbers(vec![1, 2, 3, 4, 5]);
/// ```
fn add_some_numbers(numbers: Vec<isize>) -> isize {
    numbers.into_iter().sum::<isize>()
}

fn main() {
    let file_path = "sum.txt";
    println!("Let's add some numbers. 2 + 2 = {}", 2 + 2);

    let sum = add_some_numbers(vec![1, 1]);

    let equation = format!("1 + 1 = {sum}");

    let previous_file_contents = fs::read_to_string(file_path);
    if let Ok(previous_file) = previous_file_contents {
        println!("Previous file contents are {previous_file}");
    }

    let mut file = File::options()
        .write(true)
        .open("sum.txt")
        .unwrap_or(File::create("sum.txt").unwrap());

    file.write_all(equation.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_successfully() {
        assert_eq!(add_some_numbers(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(add_some_numbers(vec![1000, 35, 51289, 964211]), 1016535);
        assert_eq!(add_some_numbers(vec![-31897, 742, 0]), -31155);
    }
}
