// program to convert numbers to corresponding month.
// Inputs: number
// process: get corresponding month
// output: month

fn number_to_month(number: u32) -> &str {
    match number {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => panic!("Invalid month number. Must be between 1 and 12 inclusive."),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_month() {
        assert_eq!(number_to_month(1), "January");
        assert_eq!(number_to_month(2), "February");
        assert_eq!(number_to_month(3), "March");
        assert_eq!(number_to_month(4), "April");
        assert_eq!(number_to_month(5), "May");
        assert_eq!(number_to_month(6), "June");
        assert_eq!(number_to_month(7), "July");
        assert_eq!(number_to_month(8), "August");
        assert_eq!(number_to_month(9), "September");
        assert_eq!(number_to_month(10), "October");
        assert_eq!(number_to_month(11), "November");
        assert_eq!(number_to_month(12), "December");

        #[should_panic]
        number_to_month(0);
        #[should_panic]
        number_to_month(13);
        #[should_panic]
        number_to_month(99);
    }
}

fn main() {
    println!("Hello, world!");
}
