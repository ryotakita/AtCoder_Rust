use proconio::{input};

fn main() {
    input!{
        a_x: f64,
        a_y: f64,
        b_x: f64,
        b_y: f64
    }

    let ratio = a_y / (a_y + b_y);
    let length = b_x - a_x;
    let mid = ratio * length;
    println!("{:.10}", mid + a_x);
}
