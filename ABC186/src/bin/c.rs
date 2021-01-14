use proconio::{input};

fn contain_7(n: i64) -> bool{
    if n < 1 { return false; }
    match n%8{
        7 => return true,
        _ => return contain_7(n/8)
    }
}
fn main() {
    input!{
        a: i64
    }

    println!("{}", (1..a+1).fold(0, 
        |acc, i| if !contain_7(i) && !i.to_string().contains("7") { acc+1 } else { acc }
    ));
    
}
