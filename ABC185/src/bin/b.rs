use proconio::{input};
use std::cmp::min;

fn main() {
    input!{
        mut maxb: i64,
        count: i64,
        end: i64,
    };
    let maxbat = maxb;
    let mut latest = 0;
    let mut vec = vec!();
    for i in 0..count{
        input!{a: [i64; 2]};
        vec.push(a);
    }

    for i in vec{
        maxb -= i[0] - latest;
        if maxb <= 0 {
            println!("No");
            return;
        };
        maxb = min(maxb+i[1] - i[0], maxbat);
        if maxb <= 0 {
            println!("No");
            return;
        };
        latest = i[1];
    }

    maxb -= end - latest;
    if maxb <= 0 {
        println!("No");
        return;
    };


    println!("Yes");

}
