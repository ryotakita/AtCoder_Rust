use proconio::{input};
use std::cmp::{min, max};

fn main() {
    input!{
        N: i64,
        mut monster: [i64; N+1],
        mut power: [i64; N]
    }

    let mut count = 0;

    for i in 0..N{
        let u = i as usize;
        let monster_first = monster[u];
        let power_first = power[u];
        monster[u] = max(monster[u] - power[u], 0);
        power[u] = max(power[u] - monster_first, 0);
        let nextmons_first = monster[u+1];
        monster[u+1] = max(monster[u+1] - power[u], 0);
        power[u] = max(power[u] - nextmons_first, 0);

        count += power_first - power[u];
    }

    println!("{}", count);


}
