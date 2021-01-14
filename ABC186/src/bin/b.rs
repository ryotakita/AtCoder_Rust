use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        h: i64,
        w: i64,
        mut array: [[i64; w]; h]
    }

    let get_min = |array: &[i64]| array.iter().fold(i64::max_value(), |acc, a| min(*a, acc));
    let minimum = array.iter().fold(i64::max_value(), |acc, a| min(get_min(a), acc));

    let get_diff = |array: &[i64]| array.iter().fold(0, |acc, a| acc + (*a - minimum));

    let count = array.iter().fold(0, |acc, a| acc + get_diff(a));

    println!("{}", count);
}
