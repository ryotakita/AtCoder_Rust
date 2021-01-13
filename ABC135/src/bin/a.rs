use proconio::{input};
use std::num;

fn main() {
    input!{
        a: i64,
        b: i64
    }

    if (a+b)%2 == 0 {
        println!("{}",(a+b)/2);
    }else{
        println!("IMPOSSIBLE");
    }

}
