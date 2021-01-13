use proconio::{input};

fn main() {
    input!{
        N: i32,
        first: i64,
        s: String 
    }

    println!("{}",s.chars().fold(first, |acc, a| if a == 'o' {acc + 1} else { if acc != 0 {acc - 1} else {acc} }));
}
