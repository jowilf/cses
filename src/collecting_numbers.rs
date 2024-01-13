use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line).expect("-error-");
    let n: i32 = line.lines().next().unwrap().parse().expect("Invalid input");
    line = "".to_string();
    stdin().read_line(&mut line).expect("-error-");
    let arr: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let mut indexes: [i32; 200001] = [0; 200001];
    for (i, v) in arr.into_iter().enumerate() {
        indexes[v as usize] = (i + 1) as i32;
    }
    let mut ans = 1;
    let mut previous_idx = -1;
    for i in 1..(n + 1) {
        // println!(" {}, {}", ans, indexes[i as usize]);
        if indexes[i as usize] < previous_idx {
            ans += 1;
        }
        previous_idx = indexes[i as usize];
    }
    println!("{}", ans)
}
