use std::env;
use std::fmt;
use std::process;

fn main() {
    // CLIから下限と上限の2引数を受け取る
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <lower> <upper>", args[0]);
        process::exit(1);
    }
    let lower: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number: {}", args[1]);
            process::exit(1);
        }
    };
    let upper: i32 = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Invalid number: {}", args[2]);
            process::exit(1);
        }
    };
    match ClosedRange::new(lower, upper) {
        Ok(range) => println!("{}", range),
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    }
}

struct ClosedRange {
    lower: i32,
    upper: i32,
}

impl ClosedRange {
    fn new(lower: i32, upper: i32) -> Result<Self, String> {
        if lower > upper {
            return Err("Lower bound cannot be greater than upper bound".to_string());
        }
        Ok(ClosedRange { lower, upper })
    }

    fn contains(&self, value: i32) -> bool {
        value >= self.lower && value <= self.upper
    }

    fn is_equal(&self, other: &ClosedRange) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }

    fn is_subset(&self, other: &ClosedRange) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

impl fmt::Display for ClosedRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.lower, self.upper)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let range = ClosedRange::new(1, 10).unwrap();
        assert_eq!(range.lower, 1);
        assert_eq!(range.upper, 10);
    }

    #[test]
    fn test_init_error() {
        let result = ClosedRange::new(10, 1);
        assert!(result.is_err());
        let message = result.err().unwrap();
        assert_eq!(message, "Lower bound cannot be greater than upper bound");
    }

    #[test]
    fn test_contains() {
        let range = ClosedRange::new(1, 10).unwrap();
        assert!(range.contains(1));
        assert!(range.contains(5));
        assert!(range.contains(10));
        assert!(!range.contains(0));
        assert!(!range.contains(11));
    }

    #[test]
    fn test_is_equal() {
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(1, 10).unwrap();
        let range3 = ClosedRange::new(2, 10).unwrap();
        assert!(range1.is_equal(&range2));
        assert!(!range1.is_equal(&range3));
    }

    #[test]
    fn test_is_subset() {
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(1, 20).unwrap();
        let range3 = ClosedRange::new(5, 8).unwrap();
        assert!(range1.is_subset(&range2));
        assert!(!range1.is_subset(&range3));
    }

    #[test]
    fn test_display() {
        let range = ClosedRange::new(1, 10).unwrap();
        assert_eq!(format!("{}", range), "[1, 10]");
    }
}
