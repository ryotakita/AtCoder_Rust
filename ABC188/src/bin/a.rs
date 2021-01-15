use proconio::{input};

fn can_win(min: i64 ,max: i64) ->bool{
    max - min < 3
}

fn main() {
    input! {
        x: i64,
        y: i64,
    }

    if x < y{
        if can_win(x, y){
            println!("Yes");
        }else{
            println!("No");
        }
    }else{
        if can_win(y, x){
            println!("Yes");
        }else{
            println!("No");
        }
    }
}