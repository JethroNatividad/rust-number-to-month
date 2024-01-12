// program to convert numbers to corresponding month.
// Inputs: number
// process: get corresponding month
// output: month

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_to_month() {
        assert_eq!(test_number_to_month(0), "Invalid Month");
        assert_eq!(test_number_to_month(1), "January");
        assert_eq!(test_number_to_month(2), "February");
        assert_eq!(test_number_to_month(3), "March");
        assert_eq!(test_number_to_month(4), "April");
        assert_eq!(test_number_to_month(5), "May");
        assert_eq!(test_number_to_month(6), "June");
        assert_eq!(test_number_to_month(7), "July");
        assert_eq!(test_number_to_month(8), "August");
        assert_eq!(test_number_to_month(9), "September");
        assert_eq!(test_number_to_month(10), "October");
        assert_eq!(test_number_to_month(11), "November");
        assert_eq!(test_number_to_month(12), "December");
        assert_eq!(test_number_to_month(99), "Invalid Month");
    }
}

fn main() {
    println!("Hello, world!");
}
