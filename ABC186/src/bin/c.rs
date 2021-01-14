///問題文
///高橋君は 7 が嫌いです。
///1 以上 N 以下の整数のうち、
///10 進法で表しても 8 進法で表しても 
///7 を含まないような数はいくつありますか？

///制約
///1 ? N ? 10^5
///N は整数である。

use proconio::{input};

fn contain_7(n: i64) -> bool{
    if n < 1 { 
        false
    }else{
        match n%8{
            7 => true,
            _ => contain_7(n/8)
        }
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
