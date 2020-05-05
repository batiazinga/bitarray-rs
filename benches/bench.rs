#![feature(test)]
extern crate test;

use bitarray::Array;
use test::{black_box, Bencher};

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
                black_box(a.get(i, j));
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
        black_box(a.get(i, j));
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
    let a: Vec<bool> = vec![false; m * n];
    // iterate
    b.iter(|| {
        for i in 0..m {
            for j in 0..n {
                black_box(a[i * 100 + j]);
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
    let a: Vec<bool> = vec![false; m * n];
    // iterate
    b.iter(|| {
        black_box(a[i * n + j]);
    });
}

#[bench]
fn bench_set_all_bool(b: &mut Bencher) {
    // init
    let m = 500usize;
    let n = 100usize;
    let mut a: Vec<bool> = vec![false; m * n];
    // iterate
    b.iter(|| {
        for i in 0..m {
            for j in 0..n {
                a[i * n + j] = true;
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
    let mut a: Vec<bool> = vec![false; m * n];
    // iterate
    b.iter(|| {
        a[i * n + j] = true;
    });
}
