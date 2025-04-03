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
        target: ClosedRange,    // サブセットかどうか判定される対象の閉区間
        container: ClosedRange, // 包含する側の閉区間
    },
}

impl Command {
    fn from_args(args: Vec<String>) -> Result<Self, String> {
        if args.len() < 3 {
            return Err("引数が不足しています。少なくとも2つの引数が必要です。".to_string());
        }

        // 最初の閉区間を作成
        let lower =
            parse_int(&args[1]).map_err(|_| format!("下限値として無効な整数です: {}", args[1]))?;
        let upper =
            parse_int(&args[2]).map_err(|_| format!("上限値として無効な整数です: {}", args[2]))?;
        let range = ClosedRange::new(lower, upper)?;

        // 追加の引数がない場合は表示コマンドのみ
        if args.len() == 3 {
            return Ok(Command::Display { range });
        }

        // 3番目の引数によってコマンドを決定
        match args.get(3).map(String::as_str) {
            Some("contains") if args.len() >= 5 => {
                let value = parse_int(&args[4])
                    .map_err(|_| format!("検査値として無効な整数です: {}", args[4]))?;
                Ok(Command::Contains { range, value })
            }
            Some("subset") if args.len() >= 6 => {
                let lower2 = parse_int(&args[4])
                    .map_err(|_| format!("2つ目の区間の下限値として無効な整数です: {}", args[4]))?;
                let upper2 = parse_int(&args[5])
                    .map_err(|_| format!("2つ目の区間の上限値として無効な整数です: {}", args[5]))?;
                let container_range = ClosedRange::new(lower2, upper2)?;
                Ok(Command::Subset {
                    target: range,
                    container: container_range,
                })
            }
            // 引数が不正な場合
            Some("contains") => Err("'c'コマンドには検査値が必要です。".to_string()),
            Some("subset") => {
                Err("'s'コマンドには2つ目の区間の下限値と上限値が必要です。".to_string())
            }
            Some(cmd) => Err(format!(
                "未知のコマンド'{}'です。'contains'または'subset'を指定してください。",
                cmd
            )),
            None => Err("区間の後にコマンドが指定されていません。".to_string()),
        }
    }

    fn execute(&self) {
        match self {
            Command::Display { range } => {
                println!("Range: {}", range);
            }
            Command::Contains { range, value } => {
                let contains = range.contains(*value);
                println!("{} contains {}: {}", range, value, contains);
            }
            Command::Subset { target, container } => {
                let is_subset = target.is_subset(container);
                println!("{} is subset of {}: {}", target, container, is_subset);
            }
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
            return Err("下限値が上限値より大きいため、有効な閉区間ではありません".to_string());
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
        assert_eq!(
            message,
            "下限値が上限値より大きいため、有効な閉区間ではありません"
        );
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
        let target = ClosedRange::new(1, 10).unwrap();
        let container = ClosedRange::new(1, 20).unwrap();
        // Act & Assert
        assert!(target.is_subset(&container));
    }

    #[test]
    fn test_is_not_subset() {
        // Arrange
        let target = ClosedRange::new(1, 10).unwrap();
        let container = ClosedRange::new(5, 15).unwrap();
        // Act & Assert
        assert!(!target.is_subset(&container));
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
