use proconio::{input};

fn main() {
    input!{
        a: i8
    }

    println!("{}", if a > 0 { a } else { 0 });
}
