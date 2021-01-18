use proconio::{input};
use std::cmp::{max};

fn main() {
    input!{
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut max_num = 0;
    let mut max_bef = 0;
    let mut max_a = 0;
    for j in 0..n{

        max_a = max(max_a, a[j]);
        max_num = max(max_num, max_a * b[j]);

        //println!("{}: a:{} b:{}", j+1, max_a, b[j]);
        println!("{}", max(max_bef, max_num));
        max_bef = max(max_bef, max_num);
    }
}
