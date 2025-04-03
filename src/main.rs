use clap::{Parser, Subcommand};
use std::process;

mod closed_range;
use closed_range::ClosedRange;

#[derive(Parser)]
#[command(author, version, about = "閉区間の操作を行うコマンドラインツール")]
struct Cli {
    #[arg(required = true)]
    lower: i32,

    #[arg(required = true)]
    upper: i32,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// 値が閉区間に含まれるかチェックする
    Contains { value: i32 },
    /// 閉区間が別の閉区間のサブセットかチェックする
    Subset { lower: i32, upper: i32 },
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let range = ClosedRange::new(cli.lower, cli.upper)?;

    match &cli.command {
        Some(Commands::Contains { value }) => {
            let contains = range.contains(*value);
            println!("{} contains {}: {}", range, value, contains);
        }
        Some(Commands::Subset { lower, upper }) => {
            let container = ClosedRange::new(*lower, *upper)?;
            let is_subset = range.is_subset(&container);
            println!("{} is subset of {}: {}", range, container, is_subset);
        }
        None => {
            println!("Range: {}", range);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_display() {
        // Arrange
        let cli = Cli {
            lower: 1,
            upper: 10,
            command: None,
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_contains_true() {
        // Arrange
        let cli = Cli {
            lower: 1,
            upper: 10,
            command: Some(Commands::Contains { value: 5 }),
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_contains_false() {
        // Arrange
        let cli = Cli {
            lower: 1,
            upper: 10,
            command: Some(Commands::Contains { value: 15 }),
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_subset_true() {
        // Arrange
        let cli = Cli {
            lower: 5,
            upper: 10,
            command: Some(Commands::Subset {
                lower: 1,
                upper: 15,
            }),
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_subset_false() {
        // Arrange
        let cli = Cli {
            lower: 1,
            upper: 10,
            command: Some(Commands::Subset {
                lower: 5,
                upper: 15,
            }),
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_invalid_range() {
        // Arrange
        let cli = Cli {
            lower: 10,
            upper: 1, // 無効な範囲
            command: None,
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_run_invalid_container() {
        // Arrange
        let cli = Cli {
            lower: 1,
            upper: 10,
            command: Some(Commands::Subset {
                lower: 15,
                upper: 5,
            }), // 無効なコンテナ
        };
        // Act
        let result = run(cli);
        // Assert
        assert!(result.is_err());
    }
}
