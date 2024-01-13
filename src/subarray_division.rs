use std::{collections::HashMap, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    iter = line.split_whitespace();
    let mut map: HashMap<i64, i64> = HashMap::new();
    let mut acc: i64 = 0;
    let mut ans: i64 = 0;
    for _ in 0..n {
        acc += iter.next().unwrap().parse::<i64>().unwrap();
        let p = acc.rem_euclid(n);
        if p == 0 {
            ans += map.get(&0).unwrap_or(&0) + 1;
        } else {
            ans += map.get(&(n - p)).unwrap_or(&0);
        }
        let pn = (-acc).rem_euclid(n);
        map.insert(pn, map.get(&pn).unwrap_or(&0) + 1);
        // println!("acc {}; p {}; pn {}; {:?}", acc, p, pn, map);
    }
    println!("{}", ans);
}
