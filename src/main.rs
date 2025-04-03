use std::env;
use std::fmt;
use std::num::ParseIntError;

fn main() {
    let args = env::args().collect();
    match Command::from_args(args) {
        Ok(command) => command.execute(),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

// コマンドを表現するenum
enum Command {
    // 単一の閉区間を表示する
    Display {
        range: ClosedRange,
    },
    // 値が閉区間に含まれるかチェックする
    Contains {
        range: ClosedRange,
        value: i32,
    },
    // 閉区間が別の閉区間のサブセットかチェックする
    Subset {
        range1: ClosedRange,
        range2: ClosedRange,
    },
}

impl Command {
    // コマンドラインからコマンドを解析する
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        if args.len() < 3 {
            return Err(format!(
                "Usage: {} <lower> <upper> [c <value> | s <lower2> <upper2>]",
                args[0]
            ));
        }

        // 最初の閉区間を作成
        let lower = parse_int(&args[1])?;
        let upper = parse_int(&args[2])?;
        let range = ClosedRange::new(lower, upper)?;

        // 引数の数によってコマンドを決定
        match args.len() {
            // 最初の閉区間のみ (表示コマンド)
            3 => Ok(Command::Display { range }),
            
            // 4引数以上の場合、3番目の引数によってコマンドを決定
            _ if args.len() >= 5 && args[3] == "c" => {
                let value = parse_int(&args[4])?;
                Ok(Command::Contains { range, value })
            },
            
            _ if args.len() >= 6 && args[3] == "s" => {
                let lower2 = parse_int(&args[4])?;
                let upper2 = parse_int(&args[5])?;
                let range2 = ClosedRange::new(lower2, upper2)?;
                Ok(Command::Subset { range1: range, range2 })
            },
            
            // その他の場合は最初の閉区間のみを表示
            _ => Ok(Command::Display { range }),
        }
    }

    // コマンドを実行する
    fn execute(&self) {
        match self {
            Command::Display { range } => {
                println!("range: {}", range);
            },
            Command::Contains { range, value } => {
                let contains = range.contains(*value);
                println!("{} contains {}: {}", range, value, contains);
            },
            Command::Subset { range1, range2 } => {
                let is_subset = range1.is_subset(range2);
                println!("{} is subset of {}: {}", range1, range2, is_subset);
            },
        }
    }
}

// 文字列から整数へのパースを行うヘルパー関数
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e: ParseIntError| e.to_string())
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

    fn is_subset(&self, other: &ClosedRange) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

impl fmt::Display for ClosedRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.lower, self.upper)
    }
}

impl PartialEq for ClosedRange {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower && self.upper == other.upper
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        // Act
        let range = ClosedRange::new(1, 10).unwrap();
        // Assert
        assert_eq!(range.lower, 1);
        assert_eq!(range.upper, 10);
    }

    #[test]
    fn test_init_error() {
        // Act
        let result = ClosedRange::new(10, 1);
        // Assert
        assert!(result.is_err());
        let message = result.err().unwrap();
        assert_eq!(message, "Lower bound cannot be greater than upper bound");
    }

    #[test]
    fn test_contains() {
        // Act
        let range = ClosedRange::new(1, 10).unwrap();
        // Assert
        assert!(range.contains(1));
        assert!(range.contains(5));
        assert!(range.contains(10));
        assert!(!range.contains(0));
        assert!(!range.contains(11));
    }

    #[test]
    fn test_is_equal() {
        // Arrange
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(1, 10).unwrap();
        // Act & Assert
        assert!(range1 == range2);
    }

    #[test]
    fn test_is_not_equal() {
        // Arrange
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(1, 11).unwrap();
        // Act & Assert
        assert!(range1 != range2);
    }

    #[test]
    fn test_is_subset() {
        // Arrange
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(1, 20).unwrap();
        // Act & Assert
        assert!(range1.is_subset(&range2));
    }

    #[test]
    fn test_is_not_subset() {
        // Arrange
        let range1 = ClosedRange::new(1, 10).unwrap();
        let range2 = ClosedRange::new(5, 15).unwrap();
        // Act & Assert
        assert!(!range1.is_subset(&range2));
    }

    #[test]
    fn test_display() {
        // Arrange
        let range = ClosedRange::new(1, 10).unwrap();
        // Act
        let output = format!("{}", range);
        // Assert
        assert_eq!(output, "[1, 10]");
    }
}
