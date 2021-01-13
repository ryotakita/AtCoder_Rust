use proconio::{input};
use std::mem::{swap};

fn main() {
    input!{
        N: usize,
        mut vec: [usize; N]
    }

    for i in 0..N-1{
        if vec[i] > vec[i+1]{
            let tmp = vec[i]-1;
            vec[i] = i+1;
            vec[tmp] = tmp+1;
            break;
        }
    }

    for i in 0..N{
        //println!("{}",vec[i]);
        if vec[i] != i+1{
            println!("{}","NO");
            return;
        }
    }

    println!("YES");

}
