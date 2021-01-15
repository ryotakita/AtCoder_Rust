use proconio::{input};

fn main() {
    input!(
        a:String,
        b:String,
    );

    let get_sum = |a: &String| a.chars().fold(0, |sum, a| sum + a as i32 - 48);
    println!("{}", if get_sum(&a) > get_sum(&b) { get_sum(&a) } else { get_sum(&b) });
    
}

