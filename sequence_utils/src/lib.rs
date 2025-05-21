/// Utility library for math sequences.

/// Struct that can generate math sequences like e.g. Collatz, Fibonacci, etc.
pub struct SequenceGenerator {}

impl SequenceGenerator {
    /// Computes the Collatz sequence for a given start value.
    /// 
    /// ## Arguments
    /// 
    /// * `start` - The starting value for the sequence, must be positive integer.
    /// 
    /// ## Returns
    /// 
    /// A vector of u64 values representing the Collatz sequence.
    /// 
    /// ## Example
    /// 
    /// ```
    /// use sequence_utils::SequenceGenerator;
    /// 
    /// let sequence = SequenceGenerator::collatz(6);
    /// # assert_eq!(sequence, vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    /// ```
    pub fn collatz(start: u64) -> Vec<u64> {
        if start == 0 {
            panic!("Start value cannot be 0");
        }

        let mut sequence = vec![start];
        let mut current = start;
        
        while current != 1 {
            if current % 2 == 0 {
                current = current / 2;
            } else {
                current = 3 * current + 1;
            }
            sequence.push(current);
        }
        
        sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz() {
        assert_eq!(SequenceGenerator::collatz(1), vec![1]);
        assert_eq!(SequenceGenerator::collatz(2), vec![2, 1]);
        assert_eq!(SequenceGenerator::collatz(3), vec![3, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(SequenceGenerator::collatz(6), vec![6, 3, 10, 5, 16, 8, 4, 2, 1]);
    }
}
