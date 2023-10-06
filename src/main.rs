/// Takes a list of numbers and returns the result of adding them together
///
/// # Arguments
/// * numbers - A Vec of numbers
///
/// # Examples
/// // Returns 15
/// add_some_numbers(vec![1, 2, 3, 4, 5]);
fn add_some_numbers(numbers: Vec<isize>) -> isize {
    numbers.into_iter().sum::<isize>()
}

fn main() {
    println!("1 + 1 = {}", add_some_numbers(vec![1, 1]));
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
