//! The `bitarray` crate provides two-dimensional arrays of single bit booleans.
//!
//! # Example:
//!
//! ```
//! use bitarray::Array;
//!
//! let mut a = Array::new(2, 3);
//! a.set(1, 2, true);
//!
//! assert_eq!(false, a.get(0, 0));
//! assert_eq!(true, a.get(1, 2));
//! ```

// #![feature(test)]
// extern crate test;

/// A two-dimensional array of single-bit booleans.
pub struct Array {
    n_rows: usize,
    n_columns: usize,
    content: Vec<u8>,
}

impl Array {
    /// Constructs a new `Array`.
    ///
    /// The array is initially full of `false`.
    pub fn new(n_rows: usize, n_columns: usize) -> Array {
        // compute size of the array's content
        let full_size = n_rows * n_columns;
        let size = if full_size % 8 == 0 {
            full_size / 8
        } else {
            full_size / 8 + 1
        };
        // return the array
        Array {
            n_rows: n_rows,
            n_columns: n_columns,
            content: vec![0; size],
        }
    }

    /// Reads the bit-boolean at position `(i,j)`.
    pub fn get(&self, i: usize, j: usize) -> bool {
        // check
        if i >= self.n_rows || j >= self.n_columns {
            panic!(
                "index out of bounds: ({},{}) outside of ({},{})",
                i, j, self.n_rows, self.n_columns
            );
        }
        // read boolean
        let (index, bit) = self.indexes(i, j);
        self.content[index] & (1 << bit) != 0
    }

    /// Sets the value of the bit-boolean at postition `(i,j)`.
    pub fn set(&mut self, i: usize, j: usize, val: bool) {
        // check
        if i >= self.n_rows || j >= self.n_columns {
            panic!(
                "index out of bounds: ({},{}) outside of ({},{})",
                i, j, self.n_rows, self.n_columns
            );
        }
        // set value
        let (index, bit) = self.indexes(i, j);
        if val {
            self.content[index] = self.content[index] | 1 << bit;
        } else {
            self.content[index] = self.content[index] & !(1 << bit);
        }
    }

    // helper to find index of the group of bit-booleans (u8).
    fn indexes(&self, i: usize, j: usize) -> (usize, usize) {
        let linear = i * self.n_columns + j;
        (linear / 8, linear % 8)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indexes() {
        let a = Array::new(12, 20);
        // 0,0
        let mut pair = a.indexes(0, 0);
        assert_eq!(0, pair.0);
        assert_eq!(0, pair.1);
        // 0,4
        pair = a.indexes(0, 4);
        assert_eq!(0, pair.0);
        assert_eq!(4, pair.1);
        // 0,10
        pair = a.indexes(0, 10);
        assert_eq!(1, pair.0);
        assert_eq!(2, pair.1);
        // 1,0
        pair = a.indexes(1, 0);
        assert_eq!(2, pair.0);
        assert_eq!(4, pair.1);
    }
}
