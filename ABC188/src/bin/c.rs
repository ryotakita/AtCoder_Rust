use proconio::{input};
use std::cmp::{max};

fn to_next_step(count: usize, score: &Vec<usize>) -> Vec<usize>{
    let mut score_new: Vec<usize> = vec!();
    for i in 0..count{
        score_new.push(max(score[i*2], score[i*2+1]));
    };

    if count > 2 {
        return to_next_step(count/2, &score_new);
    }else{
        return score_new;
    };
}

fn main() {
    input!{
        n: u32,
        mut score: [usize; 2usize.pow(n)]
    }

    let score_final = to_next_step(2usize.pow(n)/2, &score);

    let place_semi = score.iter().position(|a| a == score_final.iter().min().unwrap());
    println!("{}", place_semi.unwrap() + 1);
}
