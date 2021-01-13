use proconio::{input};
use std::cmp::{min};

fn main() {
    input!(
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    );
    println!("{}",min(a,b).min(c).min(d));
}
