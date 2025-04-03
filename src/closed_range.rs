use std::fmt;

pub struct ClosedRange {
    pub lower: i32,
    pub upper: i32,
}

impl ClosedRange {
    pub fn new(lower: i32, upper: i32) -> Result<Self, String> {
        if lower > upper {
            return Err("下限値が上限値より大きいため、有効な閉区間ではありません".to_string());
        }
        Ok(ClosedRange { lower, upper })
    }

    pub fn contains(&self, value: i32) -> bool {
        value >= self.lower && value <= self.upper
    }

    pub fn is_subset(&self, other: &ClosedRange) -> bool {
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
