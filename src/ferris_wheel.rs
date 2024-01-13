use std::io::stdin;

fn main() {
    let mut line = String::new();
    stdin().read_line(&mut line);
    let nx: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    let (n, x) = (nx[0], nx[1]);
    line = "".to_string();
    stdin().read_line(&mut line);
    let mut weigths: Vec<i32> = line
        .split_whitespace()
        .filter_map(|w| w.parse().ok())
        .collect();
    weigths.sort();
    let (mut i, mut j, mut ans) = (0, n - 1, 0);
    while i <= j {
        if i != j && (weigths[i as usize] + weigths[j as usize] <= x) {
            i += 1;
            j -= 1;
            ans += 1;
        } else {
            j -= 1;
            ans += 1;
        }
    }
    println!("{}", ans);
}
