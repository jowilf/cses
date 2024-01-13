use std::{collections::HashMap, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let n: i32 = iter.next().unwrap().parse().unwrap();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    iter = line.split_whitespace();
    let mut map: HashMap<i64, i64> = HashMap::new();
    let mut acc: i64 = 0;
    let mut ans: i64 = 0;
    for _ in 0..n {
        acc += iter.next().unwrap().parse::<i64>().unwrap();
        if acc == x {
            ans += 1;
        }
        ans += map.get(&(acc - x)).unwrap_or(&0);
        map.insert(acc, map.get(&acc).unwrap_or(&0) + 1);
        // println!("acc {}; {}", acc, ans);
    }
    println!("{}", ans);
}
