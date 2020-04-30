//! The 'bitarray' crate provides two-dimensional arrays of single bit booleans.
//!
//! # Example:
//!
//! ```
//! let mut a = bitarray::Array::new(2, 3);
//! a.set(1, 2, true);
//! assert_eq!(false, a.get(0, 0));
//! assert_eq!(true, a.get(1, 2));
//! ```

#![feature(test)]
extern crate test;

pub struct Array {
	n_rows: usize,
	n_columns: usize,
	content: Vec<u8>,
}

impl Array {
	// Static method to create a new array.
	// The array is initially full of 'false'.
	pub fn new(n_rows: usize, n_columns: usize) -> Array {
		// compute size of the array's content
		let full_size = n_rows * n_columns;
		let size =
			if full_size%8 == 0 {
				full_size / 8
			} else {
				full_size / 8 +1
			};
		// return the array
		Array {n_rows: n_rows, n_columns: n_columns, content: vec![0; size]}
	}

	// read the bit-boolean at position (i,j)
	pub fn get(&self, i: usize, j: usize) -> bool {
		// check
		if i >= self.n_rows || j >= self.n_columns {
			panic!("index out of bounds: ({},{}) outside of ({},{})",i, j, self.n_rows, self.n_columns);
		}
		// read boolean
		let (index, bit) = self.indexes(i, j);
		self.content[index]&(1<<bit) != 0
	}

	// set the value of the bit-boolean at postition (i,j)
	pub fn set(&mut self, i: usize, j: usize, val: bool) {
		// check
		if i >= self.n_rows || j >= self.n_columns {
			panic!("index out of bounds: ({},{}) outside of ({},{})",i, j, self.n_rows, self.n_columns);
		}
		// set value
		let (index, bit) = self.indexes(i, j);
		if val {
			self.content[index] = self.content[index] | 1<<bit;
		} else {
			self.content[index] = self.content[index] & !(1<<bit);
		}
    }
    
    // helper to find index of the group of bit-booleans (u8).
	fn indexes(&self, i: usize, j: usize) -> (usize, usize) {
		let linear = i * self.n_columns + j;
		(linear/8, linear%8)
	}
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;
	use test::Bencher;

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

	#[bench]
	fn bench_get_all(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let a = Array::new(m, n);
		// iterate
		b.iter(|| {
			for i in 0..m {
				for j in 0..n {
					let _discard = a.get(i, j);
				}
			}
		});
	}

	#[bench]
	fn bench_get_single(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let i = 250usize;
		let j = 50usize;
		let a = Array::new(m, n);
		// iterate
		b.iter(|| {
			let _discard = a.get(i, j);
		});
	}

	#[bench]
	fn bench_set_all(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let mut a = Array::new(m, n);
		// iterate
		b.iter(|| {
			for i in 0..m {
				for j in 0..n {
					a.set(i, j, true);
				}
			}
		});
	}

	#[bench]
	fn bench_set_single(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let i = 250usize;
		let j = 50usize;
		let mut a = Array::new(m, n);
		// iterate
		b.iter(|| {
			a.set(i, j, true);
		});
	}

	// Comparison with actual boolean

	#[bench]
	fn bench_get_all_bool(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let a: Vec<bool> = vec![false; m*n];
		// iterate
		b.iter(|| {
			for i in 0..m {
				for j in 0..n {
					let _discard = a[i*100+j];
				}
			}
		});
	}

	#[bench]
	fn bench_get_single_bool(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let i = 250usize;
		let j = 50usize;
		let a: Vec<bool> = vec![false; m*n];
		// iterate
		b.iter(|| {
			let _discard = a[i*n+j];
		});
	}

	#[bench]
	fn bench_set_all_bool(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let mut a: Vec<bool> = vec![false; m*n];
		// iterate
		b.iter(|| {
			for i in 0..m {
				for j in 0..n {
					a[i*n+j] = true;
				}
			}
		});
	}

	#[bench]
	fn bench_set_single_bool(b: &mut Bencher) {
		// init
		let m = 500usize;
		let n = 100usize;
		let i = 250usize;
		let j = 50usize;
		let mut a: Vec<bool> = vec![false; m*n];
		// iterate
		b.iter(|| {
			a[i*n+j] = true;
		});
	}
}
