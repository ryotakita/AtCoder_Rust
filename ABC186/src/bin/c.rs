///��蕶
///�����N�� 7 �������ł��B
///1 �ȏ� N �ȉ��̐����̂����A
///10 �i�@�ŕ\���Ă� 8 �i�@�ŕ\���Ă� 
///7 ���܂܂Ȃ��悤�Ȑ��͂�������܂����H

///����
///1 ? N ? 10^5
///N �͐����ł���B

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
