use std::{collections::HashMap, io::stdin};

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let nx: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (_n, x) = (nx[0], nx[1]);
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    let arr: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let mut values: HashMap<i32, i32> = HashMap::new();
    for (idx, v) in arr.into_iter().enumerate() {
        if values.contains_key(&(x - v)) {
            println!("{} {}", values[&(x - v)], idx + 1);
        }
        values.insert(v, (idx + 1) as i32);
    }
    println!("IMPOSSIBLE");
}
