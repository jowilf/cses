use std::io::stdin;

fn valid(arr: &Vec<i64>, x: i64, t: i64) -> bool {
    let mut sum = 0;
    for p in arr {
        sum += x / p;
    }
    return sum >= t;
}

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("");
    let mut iter = line.split_whitespace();
    let _n: i32 = iter.next().unwrap().parse().unwrap();
    let t: i64 = iter.next().unwrap().parse().unwrap();
    line = "".to_string();
    stdin().read_line(&mut line).expect("");
    let times: Vec<i64> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();


    // println!("{:?}", times.iter().min().unwrap()*t);
    let mut low = 1;
    let mut high = times.iter().min().unwrap() * t;
    while low < (high + 1) {
        let mid = (low + high) / 2;
        // println!("{} {} {}", low, high, mid);
        if !valid(&times, mid, t) {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    println!("{}", low);
}
