use std::io;
use std::io::Write;

// program to convert numbers to corresponding month.
// Inputs: number
// process: get corresponding month
// output: month

fn number_to_month(number: i32) -> Option<&'static str> {
    match number {
        1 => Some("January"),
        2 => Some("February"),
        3 => Some("March"),
        4 => Some("April"),
        5 => Some("May"),
        6 => Some("June"),
        7 => Some("July"),
        8 => Some("August"),
        9 => Some("September"),
        10 => Some("October"),
        11 => Some("November"),
        12 => Some("December"),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_month() {
        assert_eq!(number_to_month(1), Some("January"));
        assert_eq!(number_to_month(2), Some("February"));
        assert_eq!(number_to_month(3), Some("March"));
        assert_eq!(number_to_month(4), Some("April"));
        assert_eq!(number_to_month(5), Some("May"));
        assert_eq!(number_to_month(6), Some("June"));
        assert_eq!(number_to_month(7), Some("July"));
        assert_eq!(number_to_month(8), Some("August"));
        assert_eq!(number_to_month(9), Some("September"));
        assert_eq!(number_to_month(10), Some("October"));
        assert_eq!(number_to_month(11), Some("November"));
        assert_eq!(number_to_month(12), Some("December"));
        assert_eq!(number_to_month(13), None);
        assert_eq!(number_to_month(99), None);
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    // prompt number : "Please enter the number of the month: "
    // convert number to month
    // if not none, display number, else re ask
}
